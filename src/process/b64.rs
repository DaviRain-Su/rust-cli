use crate::utils::get_data;
use crate::Base64Format;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};

pub fn process_base64_encode(input: &str, format: Base64Format) -> anyhow::Result<String> {
    let mut reader = get_data(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let encode = match format {
        Base64Format::Standard => STANDARD,
        Base64Format::Urlsafe => URL_SAFE_NO_PAD,
    };
    Ok(encode.encode(buf))
}

pub fn process_base64_decode(input: &str, format: Base64Format) -> anyhow::Result<Vec<u8>> {
    let mut reader = get_data(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    // avoid trailing newline
    let buf = buf.trim();

    let decode = match format {
        Base64Format::Standard => STANDARD,
        Base64Format::Urlsafe => URL_SAFE_NO_PAD,
    };
    let decode = decode.decode(buf)?;

    Ok(decode)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        process_base64_encode(input, format).unwrap();
    }

    #[test]
    fn test_process_decode() {
        let input = "fixture/b64.txt";
        let format = Base64Format::Standard;
        process_base64_decode(input, format).unwrap();
    }
}
