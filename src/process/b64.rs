use crate::cli::base64_opt::Base64Format;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};
use std::io::Read;

pub fn process_base64_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader = read_data(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let encode = match format {
        Base64Format::Standard => STANDARD,
        Base64Format::Urlsafe => URL_SAFE_NO_PAD,
    };
    println!("{}", encode.encode(buf));
    Ok(())
}

pub fn process_base64_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader = read_data(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    // avoid trailing newline
    let buf = buf.trim();

    let decode = match format {
        Base64Format::Standard => STANDARD,
        Base64Format::Urlsafe => URL_SAFE_NO_PAD,
    };
    // TODO: decode data might not string,
    // so we need to handle this case(but for this example, we assume it is)
    println!("{}", String::from_utf8(decode.decode(buf)?)?);
    Ok(())
}

fn read_data(input: &str) -> anyhow::Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(std::fs::File::open(input)?)
    };
    Ok(reader)
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
