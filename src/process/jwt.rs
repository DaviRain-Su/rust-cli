use chrono::{Duration, Utc};
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tracing::warn;

const KEY: &[u8] = b"your-256-bit-secret";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String,
    sub: String,
    exp: usize,
}

pub fn process_jwt_sign(sub: String, aub: String, exp: String) -> anyhow::Result<String> {
    let days = exp
        .strip_suffix('d')
        .ok_or(anyhow::anyhow!("strip exp error"))?
        .parse::<i64>()?;
    let expiration = Utc::now() + Duration::days(days);

    let claims = Claims {
        aud: aub.to_owned(),
        sub: sub.to_owned(),
        exp: expiration.timestamp() as usize,
    };
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(KEY))?;

    Ok(token)
}

pub fn process_jwt_verify(token: String, aud: String) -> anyhow::Result<bool> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&[&aud]);
    match decode::<Claims>(&token, &DecodingKey::from_secret(KEY), &validation) {
        Ok(c) => {
            println!("{:?}", c);
            Ok(true)
        }
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => {
                warn!("Token is invalid");
                Ok(false)
            } // Example on how to handle a specific error
            ErrorKind::InvalidIssuer => Err(anyhow::anyhow!("Issuer is invalid")), // Example on how to handle a specific error
            _ => Err(anyhow::anyhow!("Some other errors: {err:?}")),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt() {
        use jsonwebtoken::errors::ErrorKind;
        use jsonwebtoken::{
            decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation,
        };
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize)]
        struct Claims {
            aud: String,
            sub: String,
            company: String,
            exp: u64,
        }

        let key = b"secret";
        let my_claims = Claims {
            aud: "me".to_owned(),
            sub: "b@b.com".to_owned(),
            company: "ACME".to_owned(),
            exp: 10000000000,
        };
        let token = match encode(
            &Header::default(),
            &my_claims,
            &EncodingKey::from_secret(key),
        ) {
            Ok(t) => t,
            Err(_) => panic!(), // in practice you would return the error
        };
        println!("token: {:?}", token);

        let mut validation = Validation::new(Algorithm::HS256);
        validation.sub = Some("b@b.com".to_string());
        validation.set_audience(&["me"]);
        validation.set_required_spec_claims(&["exp", "sub", "aud"]);
        let token_data = match decode::<Claims>(&token, &DecodingKey::from_secret(key), &validation)
        {
            Ok(c) => c,
            Err(err) => match *err.kind() {
                ErrorKind::InvalidToken => panic!("Token is invalid"), // Example on how to handle a specific error
                ErrorKind::InvalidIssuer => panic!("Issuer is invalid"), // Example on how to handle a specific error
                _ => panic!("Some other errors"),
            },
        };
        println!("{:?}", token_data.claims);
        println!("{:?}", token_data.header);
    }

    #[test]
    fn test_jwt_1() {
        use serde::{Deserialize, Serialize};

        use jsonwebtoken::errors::ErrorKind;
        use jsonwebtoken::{
            decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation,
        };

        #[derive(Debug, Serialize, Deserialize)]
        struct Claims {
            sub: String,
            company: String,
            exp: u64,
        }

        let my_claims = Claims {
            sub: "b@b.com".to_owned(),
            company: "ACME".to_owned(),
            exp: 10000000000,
        };
        let key = b"secret";

        let header = Header {
            kid: Some("signing_key".to_owned()),
            alg: Algorithm::HS512,
            ..Default::default()
        };

        let token = match encode(&header, &my_claims, &EncodingKey::from_secret(key)) {
            Ok(t) => t,
            Err(_) => panic!(), // in practice you would return the error
        };
        println!("{:?}", token);

        let token_data = match decode::<Claims>(
            &token,
            &DecodingKey::from_secret(key),
            &Validation::new(Algorithm::HS512),
        ) {
            Ok(c) => c,
            Err(err) => match *err.kind() {
                ErrorKind::InvalidToken => panic!(), // Example on how to handle a specific error
                _ => panic!(),
            },
        };
        println!("{:?}", token_data.claims);
        println!("{:?}", token_data.header);
    }
}
