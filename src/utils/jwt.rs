use crate::utils::api_response::ApiErrorResponse as AuthError;
use axum::async_trait;
use axum::extract::{FromRequest, RequestParts, TypedHeader};
use axum::headers::{authorization::Bearer, Authorization};
use jsonwebtoken::encode;
use jsonwebtoken::{decode, Algorithm};
use jsonwebtoken::{DecodingKey, EncodingKey};
use jsonwebtoken::{Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::ops::Add;
use std::time::SystemTime;
use time;

///fetch the JWT defined environment and assign it's value to a life
/// call on the new method of JwtEncryption keys to accept and pass down the secret to the jsonwebtoken crate EncodingKey and DecodingKey modules
pub static JWT_SECRET: Lazy<JwtEncryptionKeys> = Lazy::new(|| -> JwtEncryptionKeys {
    let secret = std::env::var("JWT_SECRET").expect("Invalid or missing JWT Secret");
    JwtEncryptionKeys::new(secret.as_bytes())
});
///defines fields in the JWT encryption and decryption payload
#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub id: String,
    pub email: String,
    pub fullname: String,
    pub exp: u64,
}

impl JwtClaims {
    /// generate token
    /// # Example
    /// ```rust
    ///  let expiration_time = set_jtw_exp(10);
    ///    //generate sample token
    /// let sample_claim: JwtClaims = JwtClaims {
    ///  id: String::from("16260b1d-1554-5b6f-a221-56ff4b34199c"),
    //      email: String::from("cout@lahpev.mg"),
    //    fullname: String::from("Jesse Rodney"),
    //  exp: expiration_time,
    ///};
    ///let token = sample_claim.generate_token();
    ///let token: String = token.unwrap();
    ///```
    pub fn generate_token(&self) -> Option<String> {
        //fetch the JWT secret
        let jwt_header = Header {
            alg: Algorithm::HS512,
            ..Default::default()
        };
        //build the user jwt token
        encode(&jwt_header, &self, &JWT_SECRET.encoding).ok()
    }
}

#[async_trait]
impl<S> FromRequest<S> for JwtClaims
where
    S: Send,
{
    type Rejection = AuthError;

    async fn from_request(req: &mut RequestParts<S>) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|err| AuthError::InvalidToken {
                    message: err.to_string(),
                })?;

        /*
         * Decode the user data
         * the encoding uses a custom algorithm,
         * reconfigure the jsonwebtoken crate to use the custom algorithm that was used for encryption
         *
         * typically, the decryption ought to be
         * Validation::default())
                .map_err(|err| AuthError::InvalidToken {
                    error: err.to_string(),
                })?;

        * how ever we will be using a custom algorithm below
         */
        let validation = Validation::new(Algorithm::HS512);
        let token_data = decode::<JwtClaims>(bearer.token(), &JWT_SECRET.decoding, &validation)
            .map_err(|err| AuthError::InvalidToken {
                message: err.to_string(),
            })?;
        Ok(token_data.claims)
    }
}

//implement Display for JwtClaims to allow easy debugging
impl Display for JwtClaims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {}\nemail: {}\nfullname: {}\nexp:{}",
            self.id, self.email, self.fullname, self.exp
        )
    }
}

///define JWT encryption and decryption secretes
pub struct JwtEncryptionKeys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl JwtEncryptionKeys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}
///Define jwt payload structure
/// the payload will have a token and a type
/// the structure will be used as the basis of sending out JTW from the server
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JwtPayload {
    pub token: String,
    pub token_type: String,
}

/// set the expiration of token
/// accept the exp as the minutes from now when the token will be invalidated
pub fn set_jwt_exp(exp: time::Duration) -> u64 {
    _set_jwt_exp(SystemTime::now(), exp)
}

// This internal function ease testing with custom now values
fn _set_jwt_exp(now: impl Into<time::OffsetDateTime>, exp: time::Duration) -> u64 {
    // unix epoch elapsed time
    let unix_epoch_elapsed_time: time::Duration = now.into() - time::OffsetDateTime::UNIX_EPOCH;

    // return the token expiration as the summation of current unix epoch elapsed time
    let hours_from_now = unix_epoch_elapsed_time.add(exp);

    // return the result as seconds
    hours_from_now.as_seconds_f64() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[test]
    fn test_jwt_encoder() {
        dotenv().ok();

        // set token to expire in 10 minutes
        let exp = time::Duration::minutes(10);
        let expiration_time = set_jwt_exp(exp);
        //generate sample token
        let sample_claim: JwtClaims = JwtClaims {
            id: String::from("16260b1d-1554-5b6f-a221-56ff4b34199c"),
            email: String::from("cout@lahpev.mg"),
            fullname: String::from("Jesse Rodney"),
            exp: expiration_time,
        };
        let token = sample_claim.generate_token();
        // let token: String = token.unwrap();

        //see if the length of the token is greater than 10
        // println!("{}", &token);
        // assert!(Some('e') == token.chars().next());
        assert!(token.is_some());
    }

    #[test]
    fn set_jwt_exp_should_return_the_exp_as_seconds_when_now_is_unix_epoch() {
        let now = time::OffsetDateTime::UNIX_EPOCH;
        let exp = time::Duration::minutes(42);

        assert_eq!(_set_jwt_exp(now, exp) as f64, exp.as_seconds_f64());
    }

    #[test]
    fn set_jwt_exp_should_return_the_seconds_between_unix_epoch_and_now_added_with_exp() {
        let timestamp = 1_000_000_000;
        let now = time::OffsetDateTime::from_unix_timestamp(timestamp).unwrap();
        let exp = time::Duration::minutes(42);
        let expected = timestamp as u64 + exp.as_seconds_f64() as u64;

        assert_eq!(_set_jwt_exp(now, exp), expected);
    }
}