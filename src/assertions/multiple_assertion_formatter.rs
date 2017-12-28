/**
 * Generate assertions for multiple records in a memory-efficient
 * manner. Instead of returning an array of strinfified tests to be
 * executed, return a single function and an array of prototype chains.
 */
use record::Record;
use serde_json;

/**
 * Returns a string which, when evaluated, should return array of booleans
 */
pub fn multiple_assertion_formatter(records: Vec<Record>) -> &'static str {
    // Create a string of array protoChain's
    let protoChains: Vec<Vec<String>> = records.into_iter().map(|e| e.protoChain).collect();

    // Stringify the `protoChains` to allow for concatenation
    let stringifiedProtoChains = serde_json::to_string(&protoChains);

    r#"
        (function() {
        var records = {stringifiedProtoChains};

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
    "#
}
