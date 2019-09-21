
pub mod the_base58 {
    use base58::{ToBase58, FromBase58};
    use rust_util::*;

    pub fn encode(src: Vec<u8>) -> XResult<String> {
        Ok((&src.to_base58()).to_string())
    }

    pub fn decode(src: String) -> XResult<Vec<u8>> {
        match src.from_base58() {
            Ok(dest) => Ok(dest),
            Err(err) => Err(rust_util::new_box_ioerror(&format!("{:?}", err))),
        }
    }
}

pub mod the_base64 {
    use base64;
    use rust_util::*;

    pub fn encode(src: Vec<u8>) -> XResult<String> {
        Ok(base64::encode(&src))
    }

    pub fn decode(src: String) -> XResult<Vec<u8>> {
        Ok(base64::decode(&src)?)
    }
}

#[cfg(test)]
mod tests {
    use super::{the_base58, the_base64};

    #[test]
    fn test_base58() {
        assert_eq!(the_base58::encode(vec![0x00, 0x01, 0x02]).unwrap(), "15T");
        assert_eq!(the_base58::decode("15T".to_string()).unwrap(), vec![0x00, 0x01, 0x02]);
    }

    #[test]
    fn test_base64() {
        assert_eq!(the_base64::encode(vec![0x00, 0x01, 0x02]).unwrap(), "AAEC");
        assert_eq!(the_base64::decode("AAEC".to_string()).unwrap(), vec![0x00, 0x01, 0x02]);
    }
}
