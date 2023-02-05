use std::collections::HashMap;
use regex::Regex;

fn hash_to_params(params: HashMap<String, String>) -> String {
    let mut encoded = String::new();

    for (k, v) in params {
        if !encoded.is_empty() {
            // If encoded isnt empty, that means that there are parameters already
            encoded = encoded + "&";
        } else {
            // If it is empty, then this is the first parameter and so we need to add '?'
            encoded = encoded + "?";
        }

        encoded = encoded + &*k + "=" + &*v;
    }

    encoded
}

pub fn fix_url(mut params: HashMap<String, String>, url: String) -> String {
    let mut final_url = url.clone();

    for hit in Regex::new("\\{(.*?)\\}").unwrap().captures_iter(&url) {
        // Parameter found
        let str = &hit[1];
        match params.remove(str) {
            None => panic!("Parameter {} needed but not found in provided params hash map {:?}", str, params),
            Some(actual) => {
                // replace the fuond parameter in the string
                final_url = final_url.replace(
                    &format!("{{{}}}", str),
                    &*actual);
            }
        }
    }

    if params.is_empty() { final_url } else {
        final_url + &hash_to_params(params)
    }
}

/*
 TODO:
  - Move into test directory
  - The length of the string is tested since the has map has no order, so the output does not always have the same order
 */
mod tests {
    use std::collections::HashMap;
    use crate::utils::{fix_url, hash_to_params};

    #[test]
    fn test_fixer() {
        struct BuildUriTest {
            input: Input,
            expected: String,
        }

        struct Input {
            params: HashMap<String, String>,
            url: String,
        }

        let tests = vec![
            BuildUriTest {
                input: Input {
                    url: "no/replacement/needed".to_string(),
                    params: HashMap::from([
                        ("interval".to_string(), "1m".to_string()),
                        ("symbol".to_string(), "BTC_USDT".to_string()),
                        ("something".to_string(), "else".to_string())
                    ]),
                },
                expected: "no/replacement/needed?interval=1m&symbol=BTC_USDT&something=else".to_string(),
            },
            BuildUriTest {
                input: Input {
                    url: "one/replacement/needed/{interval}".to_string(),
                    params: HashMap::from([
                        ("interval".to_string(), "1m".to_string()),
                        ("symbol".to_string(), "BTC_USDT".to_string()),
                        ("something".to_string(), "else".to_string())
                    ]),
                },
                expected: "one/replacement/needed/1m?symbol=BTC_USDT&something=else".to_string(),
            },
        ];

        fn build_uri_test(input: Input, expected: String) {
            let actual = fix_url(input.params, input.url);
            let mut actual_split = actual.split("?");
            let mut expected_split = expected.split("?");

            println!("{actual}");

            // Left side of the string
            assert_eq!(
                actual_split.next().unwrap(), expected_split.next().unwrap()
            );

            // Right side of the string
            assert_eq!(
                actual_split.next().unwrap().len(), expected_split.next().unwrap().len()
            )
        }

        tests.into_iter().for_each(|test| build_uri_test(test.input, test.expected))
    }

    // Tests for hash_to_params func

    #[test]
    fn encode_parameters() {
        struct EncodeParamsTest {
            input: HashMap<String, String>,
            expected: String,
        }

        let tests: Vec<EncodeParamsTest> = vec![
            EncodeParamsTest {
                input: HashMap::new(),
                expected: "".to_string(),
            },
            EncodeParamsTest {
                input: HashMap::from([("hello".to_string(), "hello".to_string())]),
                expected: "?hello=hello".to_string(),
            },
            EncodeParamsTest {
                input: HashMap::from([
                    ("hello".to_string(), "hello".to_string()),
                    ("interval".to_string(), "1m".to_string())
                ]),
                expected: "?interval=1m&hello=hello".to_string(),
            },
        ];

        fn encode_parameters_test(t: EncodeParamsTest) {
            let actual = hash_to_params(t.input);

            assert_eq!(actual.len(), t.expected.len())
        }

        tests.into_iter().for_each(
            |test| encode_parameters_test(test)
        )
    }
}
