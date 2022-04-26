use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Validation};
use pwhash::bcrypt;
use serde::{Serialize, Deserialize};

pub const JWT_SECRET: &str = "urashiba";
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: i64,
    uuid: String,
}

pub struct Error {
    pub message: String,
}

pub fn encode(secret: &String, id: &String) -> String {
    let mut header = jsonwebtoken::Header::default();
    header.typ = Some(String::from("JWT"));
    header.alg = jsonwebtoken::Algorithm::HS256;
    let claim = Claims {
        exp: (chrono::Utc::now() + chrono::Duration::seconds(100)).timestamp(),
        uuid: id.to_string(),
    };
    jsonwebtoken::encode(&header, &claim, &EncodingKey::from_secret(secret.as_ref())).unwrap()
}

pub fn hashing_password(password: &String) -> String {
    bcrypt::hash(password).unwrap()
}

pub fn authorize(secret: &String, token: String) -> Result<String, Error> {
    let decoded = jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    );
    match decoded {
        Ok(claims) => Ok(claims.claims.uuid),
        Err(_) => Err(Error {
            message: "Permission error".to_string(),
        }),
    }
}