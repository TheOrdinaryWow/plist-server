use serde::Deserialize;

use super::plist_template;

#[derive(Deserialize)]
pub struct PlistQuery {
    #[serde(rename = "bundleid")]
    pub bundle_id: String,
    pub name: String,
    pub version: String,
    #[serde(rename = "fetchurl")]
    pub fetch_url: String,
}

impl PlistQuery {
    pub fn build_template(&self) -> String {
        let mut template = String::from(plist_template::PLIST_TEMPLATE);
        template = template.replace("{bundle_id}", &self.bundle_id);
        template = template.replace("{name}", &self.name);
        template = template.replace("{version}", &self.version);
        template = template.replace("{fetch_url}", &self.fetch_url);
        template
    }
}
