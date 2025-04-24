use serde::Deserialize;

use super::plist_template;

#[derive(Deserialize)]
pub struct PlistQuery {
    pub bundleid: String,
    pub name: String,
    pub version: String,
    pub fetchurl: String,
}

impl PlistQuery {
    pub fn build_template(&self) -> String {
        let mut template = String::from(plist_template::PLIST_TEMPLATE);
        template = template.replace("{bundleid}", &self.bundleid);
        template = template.replace("{name}", &self.name);
        template = template.replace("{version}", &self.version);
        template = template.replace("{fetchurl}", &self.fetchurl);
        template
    }
}
