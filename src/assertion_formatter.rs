/// Generate javascript assertions for each API to see if it is supported
/// in a certain browser

use record::Record;

pub fn format_css_assertion(record: Record) -> String {
    let cssPropertyName = record.protoChain.get(1).expect("Out of bounds");
    let assertion = r#"
        (function () {
            // Check CSS properties
            var properties = document.body.style
            if ('{}' in properties) return true
            // Check CSS values
            var values = document.createElement('div').style;
            if ('{}' in values) return true
            return false
        })()
    "#;
    String::from("assertion")
}

pub fn format_js_assertion(record: Record) -> String {
    let cssPropertyName = record.protoChain.get(1).expect("Out of bounds");
    let remainingProtoObject = record.protoChain.into_iter().filter(|i| &i == "sadf");
    let formattedStaticProtoChain = record.protoChain.join(".");
    let lowercaseParentObject = record.protoChain
        .get(0)
        .expect("asdf")
        .to_lowercase();

    let exceptions = vec!["crypto", "Crypto"];

    //   let lowercaseTestCondition = String(
    //     lowercaseParentObject !== 'function' &&
    //     !exceptions.has(record.protoChain[0])
    //   );

    let lowercaseSupportTest = r#"
        if (${lowercaseTestCondition}) {
        ${lowercaseParentObject === 'function' ||
        lowercaseParentObject === record.protoChain[0]
            ? ''
            : `if (typeof ${lowercaseParentObject} !== 'undefined') {
            throw new Error('${record.protoChain[0]} is not supported but ${lowercaseParentObject} is supported')
            }`}
        }
    "#;

    r#"
        (function () {
        ${lowercaseSupportTest}
        try {
            // a
            if (typeof window === 'undefined') { return false }
            // a
            if (typeof ${record.protoChain[0]} === 'undefined') { return false }
            // a.b
            if (typeof ${formattedStaticProtoChain} !== 'undefined')  { return true }
            // a.prototype.b
            if (typeof ${record.protoChain[0]}.prototype !== 'undefined') {
            if (${remainingProtoObject.length} === 0) { return false }
            return typeof ${[record.protoChain[0], 'prototype'].concat(remainingProtoObject).join('.')} !== 'undefined'
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
    "#;

    String::from("asfd")
}
