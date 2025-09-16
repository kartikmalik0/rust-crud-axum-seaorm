use chrono::{ Duration, Utc };
use hyper::StatusCode;
use jsonwebtoken::{ decode, encode, DecodingKey, EncodingKey, Header, Validation };
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub email: String,
}

pub fn encode_jwt(email: String) -> Result<String, String> {
    let now = Utc::now();
    let expire = Duration::hours(24);

    let claims = Claims {
        exp: (now + expire).timestamp() as usize,
        iat: now.timestamp() as usize,
        email,
    };

    let secret = "kkdfik-kfiadfk-asdkiew84";

    // map_err to provide a relevant error message
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).map_err(|err|
        format!("Failed to encode JWT: {}", err)
    )
}

pub fn decode_jwt(token: &str) -> Result<Claims, String> {
    let secret = "kkdfik-kfiadfk-asdkiew84";
    let res = decode(token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default())
        .map(|data| data.claims)
        .map_err(|err| format!("Failed to decode JWT: {}", err));
    return res;
}
