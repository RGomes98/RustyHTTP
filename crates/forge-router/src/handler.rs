use std::{future, pin::Pin, result};

use forge_http::{HttpError, Request, Response};

pub type HandlerResult<'a> = Pin<Box<dyn Future<Output = Result<Response<'a>, HttpError>> + Send + 'a>>;
pub type Handler = Box<dyn for<'a> Fn(Request<'a>) -> HandlerResult<'a> + Send + Sync>;

pub struct OutputWrapper<T>(pub Option<T>);

pub trait IntoHandler: Send + Sync + 'static {
    fn into_handler(self) -> Handler;
}

impl<T> IntoHandler for T
where
    T: for<'a> Fn(Request<'a>) -> HandlerResult<'a> + Send + Sync + 'static,
{
    fn into_handler(self) -> Handler {
        Box::new(self)
    }
}

pub trait AsyncResolver<'a> {
    type Output: Future<Output = Result<Response<'a>, HttpError>> + Send;
    fn resolve(self) -> Self::Output;
}

impl<'a, T> AsyncResolver<'a> for OutputWrapper<T>
where
    T: Future<Output = Result<Response<'a>, HttpError>> + Send,
{
    type Output = T;
    fn resolve(mut self) -> Self::Output {
        self.0.take().expect("\"AsyncResolver\" initialized without value")
    }
}

pub trait SyncResolver<'a> {
    type Output: Future<Output = Result<Response<'a>, HttpError>> + Send;
    fn resolve(&mut self) -> Self::Output;
}

impl<'a> SyncResolver<'a> for OutputWrapper<result::Result<Response<'a>, HttpError>> {
    type Output = future::Ready<Result<Response<'a>, HttpError>>;
    fn resolve(&mut self) -> Self::Output {
        future::ready(self.0.take().expect("Value of \"SyncResolver\" consumed twice"))
    }
}
