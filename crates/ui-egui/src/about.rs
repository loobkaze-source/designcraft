//! Help › About DesignCraft: tabs About · Contributors · Models. About is the splash with the
//! Unigreen mark, version, and contact and project links (LINE first and largest); Contributors and
//! Models are the compiled-in credits (`crate::credits`).

use designcraft_engine::links;
use egui::{Color32, Rect, Sense, Stroke, pos2, vec2};
use serde_json::json;

use crate::DesignApp;
use crate::theme::{Tokens, semibold};

/// Unigreen mint, the app's brand colour.
pub const BRAND: Color32 = Color32::from_rgb(52, 211, 153);
/// A deep Unigreen green for the "Contact Unigreen on LINE" buttons (white label; no LINE artwork).
pub const CONTACT: Color32 = Color32::from_rgb(0x05, 0x96, 0x69);

/// The Unigreen three-leaf mark (viewBox 0 0 100 98), from `assets/brand/unigreen-mark.svg`:
/// each leaf's outline and its colour.
const MARK_LEAVES: [(&str, [u8; 3]); 3] = [
    (
        "M99.609375,48.449219C99.609375,75.710938,77.578125,97.8125,50.351562,97.929688C50.210938,96.386719,50.128906,94.808594,50.128906,93.21875C50.128906,89.765625,50.492188,86.40625,51.175781,83.167969C54.863281,65.765625,67.917969,51.816406,84.824219,46.859375C89.136719,45.597656,93.707031,44.914062,98.433594,44.914062C98.785156,44.914062,99.136719,44.914062,99.492188,44.925781C99.5625,46.09375,99.609375,47.257812,99.609375,48.449219Z",
        [126, 208, 145],
    ),
    (
        "M46.59375,93.21875C46.59375,94.761719,46.523438,96.28125,46.382812,97.789062C20.792969,95.878906,0.644531,74.519531,0.644531,48.449219C0.644531,47.28125,0.679688,46.117188,0.761719,44.972656C5.890625,45.230469,10.8125,46.292969,15.398438,48.035156C30.488281,53.75,41.976562,66.828125,45.484375,82.875C46.203125,86.207031,46.59375,89.671875,46.59375,93.21875Z",
        [93, 203, 245],
    ),
    (
        "M64.265625,27.242188C64.265625,38.375,58.953125,48.257812,50.71875,54.480469C42.480469,48.257812,37.167969,38.375,37.167969,27.242188C37.167969,16.109375,42.480469,6.222656,50.71875,0.00390625C58.953125,6.222656,64.265625,16.109375,64.265625,27.242188Z",
        [136, 90, 179],
    ),
];

/// The About window's tabs, in `UiState::about_tab` order (`help.about {tab}` names them in lowercase).
pub const ABOUT_TABS: [&str; 3] = ["About", "Contributors", "Models"];

/// Paint the Unigreen mark into `rect` (square, aspect ratio kept), rasterised once per size and cached.
pub fn paint_mark(ui: &egui::Ui, rect: Rect) {
    use designcraft_render::vello_cpu::{self, kurbo, peniko};
    let ctx = ui.ctx();
    let ppp = ctx.pixels_per_point();
    let px = ((rect.width().min(rect.height()) * ppp).round() as u32).clamp(8, 1024);
    let key = egui::Id::new(("unigreen_mark", px));
    let tex: Option<egui::TextureHandle> = ctx.data(|d| d.get_temp(key));
    let tex = tex.unwrap_or_else(|| {
        let mut rc = vello_cpu::RenderContext::new_with(px as u16, px as u16, vello_cpu::RenderSettings { num_threads: 0, ..Default::default() });
        // 100 x 98 units, centred vertically in the square.
        let k = px as f64 / 100.0;
        rc.set_transform(kurbo::Affine::scale(k) * kurbo::Affine::translate((0.0, 1.0)));
        for (path, [r, g, b]) in MARK_LEAVES {
            rc.set_paint(peniko::Color::from_rgba8(r, g, b, 255));
            if let Ok(p) = kurbo::BezPath::from_svg(path) {
                rc.fill_path(&p);
            }
        }
        rc.flush();
        let mut pm = vello_cpu::Pixmap::new(px as u16, px as u16);
        let mut res = vello_cpu::Resources::new();
        rc.render(&mut pm, &mut res);
        let ci = egui::ColorImage::from_rgba_premultiplied([px as usize, px as usize], pm.data_as_u8_slice());
        let t = ctx.load_texture("unigreen_mark", ci, egui::TextureOptions::LINEAR);
        ctx.data_mut(|d| d.insert_temp(key, t.clone()));
        t
    });
    let side = rect.width().min(rect.height());
    let r = Rect::from_center_size(rect.center(), vec2(side, side));
    ui.painter().image(tex.id(), r, Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE);
}

/// A neutral speech-bubble glyph (drawn in code) for the contact buttons.
pub fn paint_chat_icon(p: &egui::Painter, c: egui::Pos2, s: f32, color: Color32) {
    let body = Rect::from_center_size(c - vec2(0.0, s * 0.06), vec2(s, s * 0.72));
    p.rect_filled(body, s * 0.24, color);
    let tail = vec![
        pos2(body.left() + s * 0.22, body.bottom() - 1.0),
        pos2(body.left() + s * 0.12, body.bottom() + s * 0.24),
        pos2(body.left() + s * 0.44, body.bottom() - 1.0),
    ];
    p.add(egui::Shape::convex_polygon(tail, color, Stroke::NONE));
    let eye = s * 0.09;
    let bg = if color.r() as u32 + color.g() as u32 + color.b() as u32 > 380 { CONTACT } else { Color32::WHITE };
    p.circle_filled(body.center() - vec2(s * 0.18, 0.0), eye, bg);
    p.circle_filled(body.center() + vec2(s * 0.18, 0.0), eye, bg);
}

/// "Contact Unigreen on LINE (@unigreen.power)" in the interface language: the contact buttons' tooltip.
pub fn contact_tooltip(lang: &str) -> String {
    format!("{} ({})", crate::i18n::tr(lang, "Contact Unigreen on LINE"), links::LINE_ID)
}

/// The big "Contact Unigreen on LINE" button (at least `size`, wider for a longer label). Returns
/// true when clicked.
pub fn contact_button(ui: &mut egui::Ui, label: &str, tooltip: &str, size: egui::Vec2) -> bool {
    let icon = size.y * 0.5;
    let font = semibold((size.y * 0.42).max(11.0));
    let galley = crate::rtl::plain(ui.ctx(), label, font, Color32::WHITE);
    let total = icon + 8.0 + galley.size().x;
    let (r, resp) = ui.allocate_exact_size(vec2(size.x.max(total + size.y), size.y), Sense::click());
    let resp = resp.on_hover_text(tooltip).on_hover_cursor(egui::CursorIcon::PointingHand);
    let fill = if resp.hovered() { Color32::from_rgb(0x04, 0x78, 0x57) } else { CONTACT };
    ui.painter().rect_filled(r, size.y / 2.0, fill);
    let x0 = r.center().x - total / 2.0;
    paint_chat_icon(ui.painter(), pos2(x0 + icon / 2.0, r.center().y), icon, Color32::WHITE);
    ui.painter().galley(pos2(x0 + icon + 8.0, r.center().y - galley.size().y / 2.0), galley, Color32::WHITE);
    resp.clicked()
}

/// A text link that opens `url`. Returns true when clicked.
fn link(ui: &mut egui::Ui, text: &str, url: &str, color: Color32) -> bool {
    let resp = ui.add(egui::Label::new(egui::RichText::new(text).size(13.0).color(color).underline()).sense(Sense::click()));
    let resp = resp.on_hover_text(url).on_hover_cursor(egui::CursorIcon::PointingHand);
    resp.clicked()
}

pub fn show(app: &mut DesignApp, ctx: &egui::Context) {
    if !app.ui.about {
        return;
    }
    let t = Tokens::get(ctx);
    let mut open: Option<&'static str> = None;
    let resp = egui::Modal::new(egui::Id::new("about")).show(ctx, |ui| {
        ui.set_width(if app.ui.about_tab == 0 { 460.0 } else { 640.0 });
        ui.horizontal(|ui| {
            for (i, l) in ABOUT_TABS.iter().enumerate() {
                let i = i as u8;
                if ui.selectable_label(app.ui.about_tab == i, crate::rtl::widget(ui, crate::i18n::tr(&app.ui.language, l))).clicked() {
                    app.ui.about_tab = i;
                }
            }
        });
        ui.separator();
        if app.ui.about_tab != 0 {
            let h = 400.0_f32.min(ctx.content_rect().height() * 0.7).max(160.0);
            ui.allocate_ui_with_layout(vec2(ui.available_width(), h), egui::Layout::top_down(egui::Align::Min), |ui| {
                ui.set_min_height(h);
                if app.ui.about_tab == 1 {
                    crate::credits::contributors_ui(ui);
                } else {
                    crate::credits::models_ui(ui);
                }
            });
            ui.separator();
            ui.vertical_centered(|ui| {
                if ui.button(crate::rtl::widget(ui, crate::i18n::tr(&app.ui.language, "  Close  "))).clicked() {
                    app.ui.about = false;
                }
            });
            return;
        }
        ui.vertical_centered(|ui| {
            ui.add_space(8.0);
            let (r, _) = ui.allocate_exact_size(vec2(72.0, 72.0), Sense::hover());
            paint_mark(ui, r);
            ui.add_space(6.0);
            ui.label(crate::rtl::widget(
                ui,
                egui::RichText::new(crate::i18n::tr(&app.ui.language, "Unigreen Design")).font(semibold(26.0)).color(t.text_strong),
            ));
            crate::rtl::label(
                ui,
                egui::RichText::new(format!("{} {}", crate::i18n::tr(&app.ui.language, "Version"), env!("CARGO_PKG_VERSION")))
                    .size(12.0)
                    .color(t.text_dim),
            );
            ui.add_space(4.0);
            ui.label(crate::rtl::widget(
                ui,
                egui::RichText::new(crate::i18n::tr(&app.ui.language, "Page layout for print and screen — fast, open and scriptable."))
                    .size(13.0)
                    .color(t.text),
            ));
            ui.label(crate::rtl::widget(
                ui,
                egui::RichText::new(crate::i18n::tr(&app.ui.language, "Unigreen Design, based on DesignCraft by the ArtCraft team and community."))
                    .size(12.0)
                    .color(t.text_dim),
            ));
            ui.add_space(16.0);
            let lang = app.ui.language.clone();
            if contact_button(ui, crate::i18n::tr(&lang, "Contact Unigreen on LINE"), &contact_tooltip(&lang), vec2(300.0, 40.0)) {
                open = Some("discord");
            }
            ui.add_space(4.0);
            ui.label(crate::rtl::widget(
                ui,
                egui::RichText::new(crate::i18n::tr(&app.ui.language, "LINE @unigreen.power — help and feedback"))
                    .size(11.0)
                    .color(t.text_dim),
            ));
            ui.add_space(14.0);
            ui.horizontal(|ui| {
                // Centre the row of links.
                let w = 280.0;
                ui.add_space(((ui.available_width() - w) / 2.0).max(0.0));
                for (text, key) in [("unigreenpower.co.th", "website"), ("GitHub", "github"), ("Report an issue", "issues")] {
                    if link(ui, text, links::get(key).unwrap_or_default(), t.accent) {
                        open = Some(key);
                    }
                    ui.add_space(6.0);
                }
            });
            ui.add_space(16.0);
            ui.label(crate::rtl::widget(
                ui,
                egui::RichText::new(crate::i18n::tr(&app.ui.language, "Open source. No telemetry.")).size(11.0).color(t.text_dim),
            ));
            ui.add_space(8.0);
            if ui.button(crate::rtl::widget(ui, crate::i18n::tr(&app.ui.language, "  Close  "))).clicked() {
                app.ui.about = false;
            }
        });
    });
    if resp.should_close() {
        app.ui.about = false;
    }
    if let Some(k) = open {
        let _ = app.run(&format!("help.{k}"), json!({}));
    }
}
