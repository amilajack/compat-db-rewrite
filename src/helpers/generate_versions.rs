use super::constants::{BrowserNameToCaniuseMappings, FixedBrowserVersions};

struct Target {
  browserName: String,
  version: String,
  platform: String
}

struct VersionsToMark {
  left: Vec<String>,
  right: Vec<String>,
  middle: String
}

struct Capability {
  browserName: String,
  minVersion: f32,
  maxVersion: f32
}

// pub fn generate_versions() {}

// pub fn generate_versions_to_mark(marked_versions: Vec<String>, caniuse_id: String) -> VersionsToMark {}

// pub fn get_all_versions_of_target(caniuse_id: String) -> Vec<String> {}

// pub fn convert_browser_name_to_caniuse(caniuse: String) -> String {}

// pub fn convert_caniuse_to_browser_name(caniuse: String) -> String {}

// pub fn filter_duplicate_targets(targets: Vec<Target>) -> Vec<Target> {}

// pub fn get_capabilities() -> Capability {}
