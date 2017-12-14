extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub city: String
}

pub fn write_json() -> Address {
    let mut address = Address {
        street: "10 Downing Street".to_owned(),
        city: "London".to_owned(),
    };

    address.city.push_str("312");
    address
}
