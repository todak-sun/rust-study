use super::http::{HttpStatusCode, Method, Request, Response};

use super::server::Handler;
pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(HttpStatusCode::Ok, Some("<h1>Welcome</h1>".to_string())),
                "/hello" => Response::new(HttpStatusCode::Ok, Some("<h1>Hello</h1>".to_string())),
                _ => Response::new(
                    HttpStatusCode::NotFound,
                    Some(format!("<h1>NOT FOUND PAGE - {}</h1>", request.path())),
                ),
            },
            _ => Response::new(
                HttpStatusCode::NotFound,
                Some("<h1>NOT FOUND</h1>".to_string()),
            ),
        }
    }
}
