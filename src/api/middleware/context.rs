use axum::{body::Body, extract::Request, middleware::Next, response::Response};
use uuid::Uuid;

use errors::Result;

use crate::api::errors;

#[derive(Clone, Debug)]
pub struct Context {
    request_id: String,
}

impl Context {
    pub fn new(request_id: Option<String>) -> Self {
        Self {
            request_id: request_id.unwrap_or(Uuid::new_v4().to_string()),
        }
    }

    pub fn get_request_id(&self) -> String {
        self.request_id.clone()
    }
}

pub async fn context_resolver(mut req: Request<Body>, next: Next) -> Result<Response<Body>> {
    add_context_to_request(&mut req);

    Ok(next.run(req).await)
}

fn add_context_to_request(req: &mut Request<Body>) {
    let request_id = req
        .headers()
        .get("X-REQUEST-ID")
        .and_then(|header_value| header_value.to_str().ok())
        .map(|v| v.to_string());

    req.extensions_mut().insert(Context::new(request_id));
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{HeaderName, HeaderValue},
    };
    use std::str::FromStr;

    #[test]
    fn adds_context_with_request_id_from_header() {
        let mut req = Request::new(Body::empty());
        req.headers_mut()
            .insert("X-REQUEST-ID", HeaderValue::from_str("12345").unwrap());

        add_context_to_request(&mut req);

        let context = req.extensions().get::<Context>().unwrap();
        assert_eq!(context.request_id, "12345".to_string());

        assert_eq!(context.get_request_id(), "12345".to_string());
    }

    #[test]
    fn adds_context_with_generated_request_id_when_header_absent() {
        let mut req = Request::new(Body::empty());

        add_context_to_request(&mut req);

        let context = req.extensions().get::<Context>().unwrap();
        assert!(!context.request_id.is_empty());
        assert!(
            uuid::Uuid::parse_str(&context.request_id).is_ok(),
            "request_id should be a valid UUID"
        );
    }

    #[tokio::test]
    async fn context_resolves_request_id_case_insensitively() {
        let header_variations = vec![
            "X-REQUEST-ID",
            "x-request-id",
            "X-Request-Id",
            "x-REQUest-iD",
        ];

        for header_name in header_variations {
            let mut req = Request::new(Body::empty());
            req.headers_mut().insert(
                HeaderName::from_str(header_name).unwrap(),
                HeaderValue::from_static("test-id"),
            );

            add_context_to_request(&mut req);

            let context = req.extensions().get::<Context>().unwrap();
            assert_eq!(
                context.request_id, "test-id",
                "Failed for header: {}",
                header_name
            );
        }
    }
}
