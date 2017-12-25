use std::collections::HashMap;
use dotenv::dotenv;
use std::env;
use serde_json;
use reqwest;

#[derive(Serialize, Deserialize)]
pub struct TestResponse {
    pub test_name: String,
    pub passed: bool,
}

pub struct TestInitiator {}

impl TestInitiator {
    pub fn get_jobs() {}

    pub fn make_request() -> Result<HashMap<String, bool>, reqwest::Error> {
        dotenv().ok();

        let username = env::var_os("SAUCE_USERNAME").unwrap().into_string().unwrap();
        let password = env::var_os("SAUCE_ACCESS_KEY").unwrap().into_string().unwrap();
        let url = format!("https://{}:{}@saucelabs.com/rest/v1/users/{}",
                          username,
                          password,
                          username);
        let url = reqwest::Url::parse(&url).expect("Foo");
        let mut response = reqwest::get(url)?;

        let mut map: HashMap<String, bool> = HashMap::new();
        let request: Vec<TestResponse> =
            serde_json::from_str(&response.text()?).expect("Could not parse JSON");

        for r in request {
            map.insert(r.test_name, r.passed);
        }

        Ok(map)
    }
}
