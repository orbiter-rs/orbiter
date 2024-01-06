use super::config::Payload;

pub fn get_listing(
    payloads: &[Payload],
    scope: &ListingScope,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    Ok(match scope {
        ListingScope::Effective => payloads.iter().map(|p| p.id.to_owned()).collect(),
        ListingScope::All => payloads.iter().map(|p| p.id.to_owned()).collect(),
    })
}

#[derive(Debug)]
pub enum ListingScope {
    Effective,
    All,
}

impl From<&Option<String>> for ListingScope {
    fn from(s: &Option<String>) -> Self {
        if let Some(s) = s {
            match s.to_lowercase().as_str() {
                "effective" => ListingScope::Effective,
                "all" => ListingScope::All,
                _ => ListingScope::Effective,
            }
        } else {
            ListingScope::Effective
        }
    }
}
