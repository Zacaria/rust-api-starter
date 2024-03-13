use axum::http::{Request, Response};
use futures_util::future::BoxFuture;
use std::task::{Context, Poll};
use tokio::time::Instant;
use tower::Service;
use tracing::info;

use super::context;

#[derive(Clone)]
pub struct AddRequestInfo;

impl<S> tower::Layer<S> for AddRequestInfo {
    type Service = AddRequestInfoMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AddRequestInfoMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct AddRequestInfoMiddleware<S> {
    pub inner: S,
}

// Implement tower middleware
impl<S, B, T> Service<Request<B>> for AddRequestInfoMiddleware<S>
where
    S: Service<Request<B>, Response = Response<T>> + Send,
    S::Future: Send + 'static,
    B: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    // Intercepts a request
    fn call(&mut self, req: Request<B>) -> Self::Future
    where
        <S as Service<axum::http::Request<B>>>::Future: Send,
    {
        let start = Instant::now();
        let method = req.method().to_string();
        let uri = req.uri().to_string();
        let request_id = req.extensions().get::<context::Context>().cloned();

        let fut = self.inner.call(req);

        // Box::pin necessary to return future
        // async move is a future that captures variables
        // => This returns a future
        Box::pin(async move {
            // awaits request result
            let response = fut.await?;
            let duration = start.elapsed().as_millis();

            // logs data
            info!(
                method = %method,
                uri = %uri,
                status = %response.status(),
                duration = ?duration,
                request_id = ?request_id,
                // additional data can be logged here
            );

            // forward response
            Ok(response)
        })
    }
}
