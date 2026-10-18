//! Help menu: community and project links (the frontend opens `Event::OpenUrl`).

use serde_json::{Value, json};

use super::{CommandSpec, always, str_p};
use crate::links;
use crate::{Event, Result, Session, cmd};

fn open(s: &mut Session, url: String) -> Result<Value> {
    s.events.push(Event::OpenUrl(url.clone()));
    Ok(json!({"url": url}))
}

fn sibling(s: &mut Session, p: &Value) -> Result<Value> {
    let slug = str_p(p, "app").unwrap_or("photocraft").to_ascii_lowercase();
    if str_p(p, "kind") == Some("github") { open(s, links::github_of(&slug)) } else { open(s, links::page_of(&slug)) }
}

pub fn specs() -> Vec<CommandSpec> {
    vec![
        cmd!("help.website", "Kojalytics Website", ["Help"], None, "{}", always, |s, _| open(s, links::WEBSITE.into())),
        cmd!("help.appPage", "EffectKoja Home Page", ["Help"], None, "{}", always, |s, _| open(s, links::APP_PAGE.into())),
        cmd!("help.onlineTutorials", "Online Tutorials...", ["Help"], None, "{}", always, |s, _| open(s, links::APP_PAGE.into())),
        cmd!("help.inAppTutorials", "In-App Tutorials...", ["Help"], None, "{} → the Home screen's Learn tab", always, |s, p| super::frontend(
            s,
            "help.inAppTutorials",
            p
        )),
        cmd!("help.reportIssue", "Provide Feedback...", ["Help"], None, "{}", always, |s, _| open(s, links::ISSUES.into())),
        cmd!(
            "help.sibling",
            "Other Kojalytics Apps",
            [],
            None,
            "{app: photocraft|vectorcraft|filmcraft|lightcraft|pdfcraft|designcraft, kind?: page|github}",
            always,
            sibling
        ),
    ]
}
