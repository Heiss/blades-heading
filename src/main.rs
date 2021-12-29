use blades::Page;
use rayon::prelude::*;
use regex::Captures;
use regex::Regex;
use std::io::Read;

/// Convert the given string to a valid HTML element ID.
/// The only restriction is that the ID must not contain any ASCII whitespace.
pub fn normalize_id(content: &str) -> String {
    content
        .chars()
        .filter_map(|ch| {
            if ch.is_alphanumeric() || ch == '_' || ch == '-' {
                Some(ch.to_ascii_lowercase())
            } else if ch.is_whitespace() {
                Some('-')
            } else {
                None
            }
        })
        .collect::<String>()
}

/// Generate an ID for use with anchors which is derived from a "normalised"
/// string.
pub fn id_from_content(content: &str) -> String {
    let mut content = content.to_string();

    // Skip any tags or html-encoded stuff
    const REPL_SUB: &[&str] = &[
        "<em>",
        "</em>",
        "<code>",
        "</code>",
        "<strong>",
        "</strong>",
        "&lt;",
        "&gt;",
        "&amp;",
        "&#39;",
        "&quot;",
    ];
    for sub in REPL_SUB {
        content = content.replace(sub, "");
    }

    // Remove spaces and hashes indicating a header
    let trimmed = content.trim().trim_start_matches('#').trim();

    normalize_id(trimmed)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut source = Vec::new();
    std::io::stdin().read_to_end(&mut source)?;
    let mut pages: Vec<Page> = serde_json::from_slice(&source)?;

    let re =
        Regex::new(r"(?m)#+ +(?P<name>\w[\w| ]+\w) *(\{ *(?P<attr>[#|.][\w|\s|\.|#]+\w) *\})?").unwrap();

    pages.par_iter_mut().for_each(|mut page| {
        let content = page.content.to_string();

        let result = re.replace_all(&content, |cap: &Captures| {
            if let Some(name) = cap.name("name") {
                let name = name.as_str();
                let mut identifier: Option<String> = None;
                let mut classes = vec![".header"];

                if let Some(attr) = cap.name("attr") {
                    attr.as_str().split_whitespace().for_each(|v| {
                        if v.starts_with("#") {
                            identifier = Some(v.replacen("#", "", 1));
                        }

                        if v.starts_with(".") {
                            classes.push(v);
                        }
                    })
                }

                if identifier == None {
                    identifier = Some(id_from_content(name));
                }

                let attr = format!("#{}", identifier.unwrap());
                classes.insert(0, &attr);
                format!("# {} {{{}}}", name, classes.join(" "))
            } else {
                panic!("Error in blades-heading. There were no name in heading");
            }
        });
        page.content = result.to_string().into();
    });

    serde_json::to_writer(std::io::stdout(), &pages)?;
    Ok(())
}
