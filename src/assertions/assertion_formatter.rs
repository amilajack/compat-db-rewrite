/// Generate javascript assertions for each API to see if it is supported
/// in a certain browser
use liquid;
use std::collections::HashMap;
use record::Record;
use super::render;

pub fn format_css_assertion(record: Record) -> String {
    let mut map: HashMap<&str, liquid::Value> = HashMap::new();
    let css_property_name = record.protoChain.get(1).expect("Out of bounds");
    map.insert("css_property_name", liquid::Value::Str(css_property_name.to_string()));

    render(
        "(function () {
            // Check CSS properties
            var properties = document.body.style
            if ('{{css_property_name}}' in properties) return true
            // Check CSS values
            var values = document.createElement('div').style;
            if ('{{css_property_name}}' in values) return true
            return false
        })()",
        &map
    )
}

pub fn format_js_assertion(record: Record) -> String {
    let css_property_name = record.protoChain.get(1).expect("Out of bounds");
    let remaining_proto_object: Vec<String> = record.protoChain.clone().split_off(1);
    let formatted_static_proto_chain = record.protoChain.join(".");
    let lowercase_parent_object = record.protoChain
        .get(0)
        .expect("Index out of bounds")
        .to_lowercase();

    let exceptions = ["crypot", "Crypto"];
    let lowercase_test_condition =
        lowercase_parent_object != "function" &&
        !(
            exceptions[0] == record.protoChain.get(0).unwrap() ||
            exceptions[1] == record.protoChain.get(0).unwrap()
        );

    let mut map = HashMap::new();
    map.insert("lowercase_parent_object", liquid::Value::Str(lowercase_parent_object));
    map.insert("lowercase_test_condition", liquid::Value::Str(lowercase_test_condition.to_string()));

    let lowercase_support_test = r#"
        if ({{lowercase_test_condition}}) {
            {{foo}}
                ? ''
                : `if (typeof {{lowercase_parent_object}} !== 'undefined') {
                throw new Error('{record.protoChain[0]} is not supported but {lowercase_parent_object} is supported')
                }`}
        }
    "#;

    render(
        "(function () {
            {lowercase_support_test}
            try {
                // a
                if (typeof window === 'undefined') { return false }
                // a
                if (typeof {record.protoChain[0]} === 'undefined') { return false }
                // a.b
                if (typeof {formatted_static_protoChain} !== 'undefined')  { return true }
                // a.prototype.b
                if (typeof {record.protoChain[0]}.prototype !== 'undefined') {
                if ({remaining_proto_object.length} === 0) { return false }
                return typeof {[record.protoChain[0], 'prototype'].concat(remaining_proto_object).join('.')} !== 'undefined'
                }
                return false
            } catch (e) {
                // TypeError thrown on property access and all prototypes are defined,
                // item usually experiences getter error
                // Ex. HTMLInputElement.prototype.indeterminate
                // -> 'The HTMLInputElement.indeterminate getter can only be used on instances of HTMLInputElement'
                return (e instanceof TypeError)
            }
        })()",
        &map
    )
}

struct CssStruct {
    apiIsSupported: String,
    allCSSValues: String,
    allCSSProperties: String,
}

struct JsStruct {
    apiIsSupported: String,
    determineASTNodeType: String,
    determineIsStatic: String,
}

enum Assertions {
    JsStruct(JsStruct),
    CssStruct(CssStruct),
}

pub fn assertion_formatter<'a>(record: Record) -> HashMap<&'a str, &'a str> {
    match record.apiType {
        css_api => {
            let mut map = HashMap::new();
            map.insert("apiIsSupported", "");
            map.insert("allCssValues", "");
            map.insert("allCssProperties", "");
            map
        }
        js_api => {
            let mut map = HashMap::new();
            map.insert("apiIsSupported", "");
            map.insert("determineAstNodeType", "");
            map.insert("determineIsStatic", "");
            map
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use record;

    #[test]
    fn format_js_assertion_has_assertion() {
        let record = Record {
            astNodeTypes: vec![record::AstNodeTypes::MemberExpression],
            id: "console.log".to_string(),
            name: "console.log".to_string(),
            protoChain: vec!["console".to_string(), "log".to_string()],
            protoChainId: "console.log".to_string(),
            isStatic: true,
            specNames: vec!["foo".to_string(), "bar".to_string()],
            specIsFinished: true,
            apiType: record::ApiType::js_api,
        };
        let result = format_js_assertion(record);
        assert!(result.contains("window"));
    }
}
