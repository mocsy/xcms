use actix_web::error::{Error, Result};
use actix_web::middleware::identity::{Identity, IdentityPolicy};
use actix_web::middleware::Response;
use actix_web::{HttpRequest, HttpResponse};
use futures::future::{ok, FutureResult};

/// The ``token`` certifies that the ``claim`` of the ``entity`` is valid only if ``token`` is ``approved``
#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct TokenIdentity {
    pub token: String,
    pub claim: String,
}

impl Identity for TokenIdentity {
    /// Return the claimed identity of the user associated request or
    /// ``None`` if no identity can be found associated with the request.
    fn identity(&self) -> Option<&str> {
        if !self.claim.is_empty() {
            return Some(&self.claim);
        }
        None
    }

    /// Remember identity.
    fn remember(&mut self, key: String) {
        self.claim = key;
    }

    /// This method is used to 'forget' the current identity on subsequent
    /// requests.
    fn forget(&mut self) {
        self.claim = String::new();
    }

    /// Write session to storage backend.
    fn write(&mut self, resp: HttpResponse) -> Result<Response> {
        Ok(Response::Done(resp))
    }
}

pub struct TokenIdentityPolicy {}
impl<S> IdentityPolicy<S> for TokenIdentityPolicy {
    type Identity = TokenIdentity;
    type Future = FutureResult<TokenIdentity, Error>;

    fn from_request(&self, _req: &HttpRequest<S>) -> Self::Future {
        // let mut config = Config::default();
        // config.realm("Restricted area");
        // config.scope("openid profile email");
        // let auth = match BearerAuth::from_request(&req, &config) {
        //     Ok(auth) => auth,
        //     Err(_) => (),
        // }

        ok(TokenIdentity {
            token: "91".to_string(),
            claim: "Fakó".to_string(),
        })
    }
}
