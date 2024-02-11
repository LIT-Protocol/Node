use lit_core::error::{Result, Unexpected};
use uuid::Uuid;

use crate::errors::parser_err;

pub fn parse_api_key(key: &String) -> Result<bool> {
    let key = String::from(key);
    let index = key.rfind('_').expect_or_err("Could not find character - in string")?;
    let parts = key.split_at(index);
    let uuid = parts.0;

    let uuid = Uuid::parse_str(uuid);
    match uuid {
        Ok(_) => Ok(true),
        Err(e) => Err(parser_err(e, Some("Could not parse api key".to_string()))),
    }
}

#[cfg(test)]

mod tests {
    use super::parse_api_key;
    #[test]
    fn should_parse_uuid() {
        let test_key = "67e55044-10b1-426f-9247-bb680e5fe0c8_myApp";
        let res = parse_api_key(&test_key.to_string()).expect("Invalid api token provided");

        assert_eq!(res, true);
    }
}
