

mod utils {
    use std::convert::TryInto;
    use std::string::FromUtf8Error;
    use std::num::TryFromIntError;

    pub fn convert_to_string(arr: [u8; 4]) -> Result<String, FromUtf8Error> {
        String::from_utf8(arr.to_vec())
    }

    pub fn convert_to_integer(val: i32) -> Result<String, TryFromIntError> {
        Ok(val.to_string())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_convert_to_string_valid_utf8() {
            let arr = [104, 101, 108, 108];
            assert_eq!(convert_to_string(arr).unwrap(), "hell");
        }

        #[test]
        fn test_convert_to_string_invalid_utf8() {
            let arr = [255, 255, 255, 255];
            assert!(convert_to_string(arr).is_err());
        }

        #[test]
        fn test_convert_to_integer() {
            let val = 123;
            assert_eq!(convert_to_integer(val).unwrap(), "123");
        }

        #[test]
        fn test_convert_to_integer_negative() {
            let val = -123;
            assert_eq!(convert_to_integer(val).unwrap(), "-123");
        }

        #[test]
        fn test_convert_to_integer_too_large() {
            let val = std::i32::MAX;
            assert_eq!(convert_to_integer(val).unwrap(), "2147483647");
        }
    }
}