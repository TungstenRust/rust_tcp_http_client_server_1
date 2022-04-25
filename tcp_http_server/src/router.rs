use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use http::{http_request, http_request::HttpRequest, http_response::HttpResponse};
use std::io::prelude::*;
pub struct Router;
impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        match req.method {
            // If GET request
                http_request::Method::Get => match &req.resource {
                http_request::Resource::Path(s) => {
                // Parse the URI
                let route: Vec<&str> = s.split("/").collect();
                match route[1] {
                //if the route begins with /api, invoke Web service
                "api" => {
                let resp: HttpResponse = WebServiceHandler::handle(&req);
                let _ = resp.send_response(stream);
                }
                // Else, invoke static page handler
                _ => {
                let resp: HttpResponse = StaticPageHandler::handle(&req);
                let _ = resp.send_response(stream);
                }
                }
                }
            },
            // If method is not GET request, return 404 page
            _ => {
                let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}