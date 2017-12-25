pub struct VendorPrefixMappings {
    chrome: String,
    safari: String,
    firefox: String,
    edge: String,
    ie: String,
}

pub fn prefixes() -> Vec<String> {
    vec![String::from("webkit"), String::from("moz"), String::from("ms")]
}

pub fn has_prefixes(property: String) -> bool {
    let lower_case_property = property.to_lowercase();
    prefixes().into_iter().find(|prefix| lower_case_property.contains(prefix)) != None
}
