pub mod multiple_assertion_formatter;
pub mod assertion_formatter;

use liquid;
use std::collections::{HashMap};

pub fn render(template: &str, map: &HashMap<&str, liquid::Value>) -> String {
    let template = liquid::ParserBuilder::with_liquid()
        .build()
        .parse(template)
        .unwrap();

    let mut globals = liquid::Object::new();
    for (&k, &v) in map {
        globals.insert(k.to_string(), v);
    }

    template.render(&globals).unwrap()
}
