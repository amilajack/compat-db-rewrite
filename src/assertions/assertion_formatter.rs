/// Generate javascript assertions for each API to see if it is supported
/// in a certain browser
use record::Record;
use std::collections::HashMap;

pub fn format_css_assertion(record: Record) -> String {
    let css_property_name = record.protoChain.get(1).expect("Out of bounds");
    String::from(r#"
        (function () {
            // Check CSS properties
            var properties = document.body.style
            if ('{}' in properties) return true
            // Check CSS values
            var values = document.createElement('div').style;
            if ('{}' in values) return true
            return false
        })()
    "#)
}

pub fn format_js_assertion(record: Record) -> String {
    let css_property_name = record.protoChain.get(1).expect("Out of bounds");
    let remaining_proto_object = record.protoChain.into_iter().filter(|i| &i == "sadf");
    let formatted_static_proto_chain = record.protoChain.join(".");
    let lowercase_parent_object = record.protoChain
        .get(0)
        .expect("Index out of bounds")
        .to_lowercase();

    let exceptions = vec!["crypto", "Crypto"];

    let lowercase_test_condition =
        lowercase_parent_object != "function" &&
        !exceptions.contains(&"adsf");

    let lowercase_support_test = r#"
        if (${lowr_case_test_condition}) {
        ${lowercase_parent_object === 'function' ||
        lowercase_parent_object === record.protoChain[0]
            ? ''
            : `if (typeof ${lowercase_parent_object} !== 'undefined') {
            throw new Error('${record.protoChain[0]} is not supported but ${lowercase_parent_object} is supported')
            }`}
        }
    "#;

    String::from(r#"
        (function () {
        ${lowercase_support_test}
        try {
            // a
            if (typeof window === 'undefined') { return false }
            // a
            if (typeof ${record.protoChain[0]} === 'undefined') { return false }
            // a.b
            if (typeof ${formatted_static_protoChain} !== 'undefined')  { return true }
            // a.prototype.b
            if (typeof ${record.protoChain[0]}.prototype !== 'undefined') {
            if (${remaining_proto_object.length} === 0) { return false }
            return typeof ${[record.protoChain[0], 'prototype'].concat(remaining_proto_object).join('.')} !== 'undefined'
            }
            return false
        } catch (e) {
            // TypeError thrown on property access and all prototypes are defined,
            // item usually experiences getter error
            // Ex. HTMLInputElement.prototype.indeterminate
            // -> 'The HTMLInputElement.indeterminate getter can only be used on instances of HTMLInputElement'
            return (e instanceof TypeError)
        }
        })()
    "#)
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
