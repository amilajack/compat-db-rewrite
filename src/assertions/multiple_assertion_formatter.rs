/**
 * Generate assertions for multiple records in a memory-efficient
 * manner. Instead of returning an array of strinfified tests to be
 * executed, return a single function and an array of prototype chains.
 */
use serde_json;
use liquid;
use std::collections::HashMap;
use record::Record;
use super::render;

/**
 * Returns a string which, when evaluated, should return array of booleans
 */
pub fn multiple_assertion_formatter(records: Vec<Record>) -> String {
    // Create a string of array protoChain's
    let proto_chains: Vec<Vec<String>> = records.into_iter().map(|e| e.protoChain).collect();

    // Stringify the `proto_chains` to allow for concatenation
    let stringified_proto_chains = serde_json::to_string(&proto_chains).unwrap();
    let mut map: HashMap<&str, liquid::Value> = HashMap::new();
    map.insert("stringified_proto_chains", liquid::Value::Str(stringified_proto_chains.to_string()));

    render(
        r#"
            (function() {
            var records = {{stringified_proto_chains}};

            /**
             * @param {record} Array<string>
             */
            function test(protoChain) {
                var remainingProtoObject = protoChain.filter(function(e, i) {
                    return i > 0
                });
                var formattedStaticProtoChain = protoChain.join('.');

                try {
                    // a
                    if (typeof window === 'undefined') { return false }
                    // a
                    if (typeof eval(protoChain[0]) === 'undefined') { return false }
                    // a.b
                    if (typeof eval(formattedStaticProtoChain) !== 'undefined')  { return true }
                    // a.prototype.b
                    if (typeof eval(protoChain[0] + '.prototype') !== 'undefined') {
                        if (remainingProtoObject.length === 0) { return false }
                        return (
                            typeof eval([protoChain[0], 'prototype'].concat(remainingProtoObject).join('.')) !== 'undefined'
                        )
                    }
                    return false
                    } catch (e) {
                        // TypeError thrown on property access and all prototypes are defined,
                        // item usually experiences getter error
                        // Ex. HTMLInputElement.prototype.indeterminate
                        // -> 'The HTMLInputElement.indeterminate getter can only be used on instances of HTMLInputElement'
                        return (e instanceof TypeError)
                    }
                }

                var results = [];

                for (var i = 0; i < records.length; i++) {
                    results.push(test(records[i]));
                }

                return results;
            })();
        "#,
        &map
    )
}
