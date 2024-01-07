use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::time::Duration;

const JWK_URL: &str =
    "https://www.googleapis.com/service_accounts/v1/jwk/securetoken@system.gserviceaccount.com";

#[derive(Debug)]
pub struct JwkConfiguration {
    pub jwk_url: String,
    pub audience: String,
    pub issuer: String,
}

#[derive(Debug, Deserialize)]
pub struct KeyResponse {
    pub keys: Vec<JwkKey>,
}

#[derive(Debug, Clone)]
pub struct JwkKeys {
    pub keys: Vec<JwkKey>,
    pub max_age: Duration,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JwkKey {
    pub e: String,
    pub alg: String,
    pub kty: String,
    pub kid: String,
    pub n: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FirebaseUser {
    pub provider_id: Option<String>,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub iss: String,
    pub aud: String,
    pub auth_time: u64,
    pub user_id: String,
    pub sub: String,
    pub iat: u64,
    pub exp: u64,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub firebase: FirebaseProvider,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FirebaseProvider {
    sign_in_provider: String,
    identities: Map<String, Value>,
}

pub fn verify_id_token_with_project_id(token: &str) -> Result<FirebaseUser, VerificationError> {
    let public_keys = get_public_keys().unwrap();
    let config = get_config();
    let header = decode_header(token).map_err(|_| VerificationError::UnkownKeyAlgorithm)?;

    if header.alg != Algorithm::RS256 {
        return Err(VerificationError::UnkownKeyAlgorithm);
    }

    let kid = match header.kid {
        Some(v) => v,
        None => return Err(VerificationError::NoKidHeader),
    };

    let public_key = match public_keys.keys.iter().find(|v| v.kid == kid) {
        Some(v) => v,
        None => return Err(VerificationError::NotfoundMatchKid),
    };
    let decoding_key = DecodingKey::from_rsa_components(&public_key.n, &public_key.e)
        .map_err(|_| VerificationError::CannotDecodePublicKeys)?;

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[config.audience.to_owned()]);
    validation.set_issuer(&[config.issuer.to_owned()]);

    let user = decode::<FirebaseUser>(token, &decoding_key, &validation)
        .map_err(|_| VerificationError::InvalidSignature)?
        .claims;
    Ok(user)
}

fn get_public_keys() -> Result<JwkKeys, PublicKeysError> {
    let response =
        reqwest::blocking::get(JWK_URL).map_err(|_| PublicKeysError::NoCacheControlHeader)?;

    let cache_control = match response.headers().get("Cache-Control") {
        Some(header_value) => header_value.to_str(),
        None => return Err(PublicKeysError::NoCacheControlHeader),
    };

    let max_age = match cache_control {
        Ok(v) => parse_max_age_value(v),
        Err(_) => return Err(PublicKeysError::MaxAgeValueEmpty),
    };

    let public_keys = response
        .json::<KeyResponse>()
        .map_err(|_| PublicKeysError::CannotParsePublicKey)?;

    Ok(JwkKeys {
        keys: public_keys.keys,
        max_age: max_age.unwrap_or(Duration::from_secs(60)),
    })
}

fn parse_max_age_value(cache_control_value: &str) -> Result<Duration, PublicKeysError> {
    let tokens: Vec<(&str, &str)> = cache_control_value
        .split(',')
        .map(|s| s.split('=').map(|ss| ss.trim()).collect::<Vec<&str>>())
        .map(|ss| {
            let key = ss.first().unwrap_or(&"");
            let val = ss.get(1).unwrap_or(&"");
            (*key, *val)
        })
        .collect();
    match tokens
        .iter()
        .find(|(key, _)| key.to_lowercase() == *"max-age")
    {
        None => Err(PublicKeysError::NoMaxAgeSpecified),
        Some((_, str_val)) => Ok(Duration::from_secs(
            str_val
                .parse()
                .map_err(|_| PublicKeysError::NonNumericMaxAge)?,
        )),
    }
}

fn get_config() -> JwkConfiguration {
    let project_id = "tyche-vtt";

    JwkConfiguration {
        jwk_url: JWK_URL.to_owned(),
        audience: project_id.to_owned(),
        issuer: format!("https://securetoken.google.com/{}", project_id),
    }
}

#[derive(Debug)]
pub enum VerificationError {
    InvalidSignature,
    UnkownKeyAlgorithm,
    NoKidHeader,
    NotfoundMatchKid,
    CannotDecodePublicKeys,
}

#[derive(Debug)]
pub enum PublicKeysError {
    NoCacheControlHeader,
    MaxAgeValueEmpty,
    NonNumericMaxAge,
    NoMaxAgeSpecified,
    CannotParsePublicKey,
}
