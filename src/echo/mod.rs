use restman_rs::{ApiHttpClient, backends::ureq::UreqApiHttpClient};

use crate::echo::client::EchoBackend;

pub mod client;
pub mod config;
pub mod endpoints;
mod request;

const DEFAULT_ROOT: &str = "https://echo360.net.au";
const COOKIE_NAME: &str = "PLAY_SESSION";
const AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/26.0 Safari/605.1.15";

pub struct EchoUser<C: ApiHttpClient> {
    backend: EchoBackend<C>,
}

impl<C: ApiHttpClient> EchoUser<C> {
    fn new_with_backend(token: &str, mut c: C) -> Self {
        c.set_cookie(COOKIE_NAME, token);
        Self {
            backend: EchoBackend::new_with_backend(c),
        }
    }
}

impl EchoUser<UreqApiHttpClient> {
    pub fn new(token: &str) -> Self {
        let backend = UreqApiHttpClient::new(AGENT);

        Self::new_with_backend(token, backend)
    }

    pub fn backend(&self) -> &EchoBackend<UreqApiHttpClient> {
        &self.backend
    }
}
