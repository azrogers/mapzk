use std::fmt::Debug;

use serde::Deserialize;

use super::deserializer::de::from_str;
use super::parse_error::{ParseError, ParseErrorType};

#[derive(Deserialize, Debug, PartialEq)]
pub struct SingleContainer<T: Debug + PartialEq> {
    pub val: T,
}

impl<'a, T: Debug + PartialEq> SingleContainer<T>
where
    T: Deserialize<'a>,
{
    pub fn new(val: T) -> SingleContainer<T> {
        SingleContainer { val }
    }

    pub fn expect(text: &'a str, expected: T) -> Result<(), ParseError> {
        assert_eq!(from_str::<SingleContainer<T>>(text)?.val, expected);
        Ok(())
    }
}

pub fn expect_str<'a, T: Debug + PartialEq>(text: &'a str, expected: T) -> Result<(), ParseError>
where
    T: Deserialize<'a>,
{
    assert_eq!(from_str::<T>(text)?, expected);
    Ok(())
}

pub fn expect_error<T: for<'a> Deserialize<'a> + std::fmt::Debug>(
    source: &str,
    expected_error: ParseErrorType,
) -> Result<(), ParseError> {
    let result = from_str::<T>(source);
    assert!(
        result.is_err(),
        "expected error {:?} but got result {:?}",
        expected_error,
        result.unwrap()
    );
    match result {
        Ok(_) => Ok(()),
        Err(e) => match e.error_type == expected_error {
            true => Ok(()),
            false => Err(e),
        },
    }
}
