//! Unigreen contact and project links (Help menu, About, start screen, CLI, MCP).

use serde_json::{Value, json};

/// The app's slug (its GitHub repository name).
pub const APP: &str = "designcraft";
/// Contact Unigreen on LINE (@unigreen.power). The link key stays `discord` (`help.discord`) so
/// existing scripts keep working.
pub const LINE: &str = "https://line.me/R/ti/p/@unigreen.power";
/// The LINE account, for labels and tooltips.
pub const LINE_ID: &str = "@unigreen.power";
/// The Unigreen website.
pub const WEBSITE: &str = "https://unigreenpower.co.th";
/// Unigreen Design's web page (the Unigreen website).
pub const APP_PAGE: &str = "https://unigreenpower.co.th";
/// Unigreen Design's source repository.
pub const GITHUB: &str = "https://github.com/loobkaze-source/designcraft";
/// Where to report bugs and request features.
pub const ISSUES: &str = "https://github.com/loobkaze-source/designcraft/issues";

/// All links as JSON (the `app.links` command).
pub fn all() -> Value {
    json!({
        "discord": LINE,
        "website": WEBSITE,
        "appPage": APP_PAGE,
        "github": GITHUB,
        "issues": ISSUES,
    })
}

/// A link by key (`discord` — the LINE contact, `website`, `appPage`, `github`, `issues`).
pub fn get(key: &str) -> Option<&'static str> {
    Some(match key {
        "discord" => LINE,
        "website" => WEBSITE,
        "appPage" => APP_PAGE,
        "github" => GITHUB,
        "issues" => ISSUES,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links_follow_the_app_slug() {
        assert!(GITHUB.ends_with(&format!("/loobkaze-source/{APP}")));
        assert!(LINE.ends_with(LINE_ID));
        assert!(ISSUES.starts_with(GITHUB));
        for k in ["discord", "website", "appPage", "github", "issues"] {
            assert!(get(k).is_some_and(|u| u.starts_with("https://")), "{k}");
            assert_eq!(all()[k], get(k).unwrap());
        }
    }
}
