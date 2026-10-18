//! Community and project links (shown in Help ▸, the About dialog, the start screen and the
//! header).

pub const APP_NAME: &str = "EffectKoja";
pub const APP_SLUG: &str = "effectcraft";
pub const WEBSITE: &str = "https://kojalytics.com";
pub const APP_PAGE: &str = "https://www.kojalytics.com/koja-apps";
pub const GITHUB: &str = "https://github.com/Kojalytics/effectcraft";
pub const ISSUES: &str = "https://www.kojalytics.com/contact";

/// Sibling apps of the ArtCraft family: (name, what it is, GitHub repo, app page).
pub const SIBLINGS: &[(&str, &str)] = &[
    ("PhotoKoja", "photocraft"),
    ("VectorKoja", "vectorcraft"),
    ("FilmKoja", "filmcraft"),
    ("LightKoja", "lightcraft"),
    ("PdfKoja", "pdfcraft"),
    ("DesignKoja", "designcraft"),
];

pub fn github_of(slug: &str) -> String {
    format!("https://github.com/Kojalytics/{slug}")
}
pub fn page_of(slug: &str) -> String {
    format!("https://kojalytics.com/apps/{slug}")
}
