use rand::prelude::SliceRandom;
use zxcvbn::zxcvbn;

const UPPER: &[u8] = b"ABCDEFGHIJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijklmnpqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*-_";

pub fn process_genpass(
    length: usize,
    uppercase: bool,
    lowercase: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = vec![];
    let mut chars = vec![];

    if uppercase {
        chars.extend_from_slice(UPPER);
        password.push(
            *UPPER
                .choose(&mut rng)
                .expect("UPPER won'ts be empty in this context"),
        );
    }
    if lowercase {
        chars.extend_from_slice(LOWER);
        password.push(
            *LOWER
                .choose(&mut rng)
                .expect("LOWER won'ts be empty in this context"),
        );
    }
    if number {
        chars.extend_from_slice(NUMBER);
        password.push(
            *NUMBER
                .choose(&mut rng)
                .expect("NUMBER won'ts be empty in this context"),
        );
    }
    if symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(
            *SYMBOL
                .choose(&mut rng)
                .expect("SYMBOL won'ts be empty in this context"),
        );
    }

    for _ in 0..(length - password.len()) {
        let c = chars
            .choose(&mut rng)
            .expect("chars won'ts be empty in this context");
        password.push(*c);
    }

    password.shuffle(&mut rng);

    // Make sure the password have at least one of each type

    let passwords_string = String::from_utf8(password)?;
    println!("{}", passwords_string);

    let estimate = zxcvbn(&passwords_string, &[]).unwrap();
    println!("password strength {}", estimate.score());
    Ok(())
}
