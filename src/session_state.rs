use std::future::{ready, Ready};
use actix_session::{Session, SessionExt, SessionGetError, SessionInsertError};
use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;

pub struct TypedSession(Session);

impl TypedSession {
    const USER_ID_KEY: &'static str = "user_id";

    pub fn renew(&self) {
        self.0.renew();
    }

    pub fn insert_user_id(&self, user_id: uuid::Uuid) -> Result<(), SessionInsertError> {
        self.0
            .insert(Self::USER_ID_KEY, user_id)
    }

    pub fn get_user_id(&self) -> Result<Option<uuid::Uuid>, SessionGetError> {
        self.0.get(Self::USER_ID_KEY)
    }
}

impl FromRequest for TypedSession {
    // We return the same error returned by the implementation of `FromRequest` for `Session`
    type Error = <Session as FromRequest>::Error;
    type Future = Ready<Result<TypedSession, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(Ok(TypedSession(req.get_session())))
    }
}
