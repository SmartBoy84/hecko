// bunch of boilerplate that my shitty library needs...

use restman_rs::{
    ApiBackendError, ApiHttpClient, MethodMarkerGetter,
    client::{ApiClientBackend, ApiClientServer, sync_client::ApiClient},
    request::{ValidRequest, endpoints::Endpoint},
};
use serde::Deserialize;
use thiserror::Error;

use crate::echo::{DEFAULT_ROOT, EchoUser};

#[derive(Debug, Error)]
pub enum EchoErr<C: ApiHttpClient> {
    #[error(transparent)]
    BackendErr(#[from] ApiBackendError<C>),

    #[error(transparent)]
    ApiErr(#[from] EchoApiErr),
}

#[derive(Debug, Error)]
#[error("{status}: {message}")]
pub struct EchoApiErr {
    pub status: String,
    pub message: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EchoRes<T> {
    pub status: String,
    pub message: String,
    pub data: T,
}

impl<T> Into<Result<T, EchoApiErr>> for EchoRes<T> {
    fn into(self) -> Result<T, EchoApiErr> {
        match self.status.as_str() {
            "ok" => Ok(self.data),
            _ => Err(EchoApiErr {
                status: self.status,
                message: self.message,
            }),
        }
    }
}

type EchoResult<T, C> = Result<T, EchoErr<C>>;

impl<C: ApiHttpClient> EchoUser<C> {
    pub fn request<
        T,
        R: ValidRequest<E>,
        E: Endpoint<Res = EchoRes<T>, Ser = Echo360, Payload = ()>,
    >(
        &self,
        r: &R,
    ) -> EchoResult<T, C>
    where
        E::Method: MethodMarkerGetter<C>,
    {
        let res: Result<_, _> = self.backend.request(r)?.into();
        Ok(res?)
    }
}

pub struct EchoBackend<C: ApiHttpClient>(C);

impl<C: ApiHttpClient> EchoBackend<C> {
    pub fn new_with_backend(backend: C) -> Self {
        Self(backend)
    }
}

impl<C: ApiHttpClient> ApiClientBackend<C> for EchoBackend<C> {
    fn backend(&self) -> &C {
        &self.0
    }
}

impl<C: ApiHttpClient> ApiClientServer<Echo360> for EchoBackend<C> {}

pub struct Echo360 {
    pub server: String,
}
impl restman_rs::Server for Echo360 {}

// set default backend
impl restman_rs::ConstServer for Echo360 {
    const ROOT: &str = DEFAULT_ROOT;
}

impl restman_rs::DynamicServer for Echo360 {
    fn get_root(&self) -> &str {
        &self.server
    }
}
