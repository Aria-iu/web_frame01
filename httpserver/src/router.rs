use std::io::Write;

use http::{httprequest::{self, HttpRequest}, httpresponse::HttpResponse};

use crate::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};

pub struct Router;
impl Router{
    pub fn route(req:HttpRequest,stream:&mut impl Write) -> (){
        match req.method {
            httprequest::Method::Get => {
                match req.resource {
                    httprequest::Resource::Path(ref s) => {
                        let route: Vec<&str> = s.split("/").collect();
                        match route[1] {
                            "api" => {
                                let response: HttpResponse = WebServiceHandler::handle(&req);
                                let _ = response.send_response(stream);
                            },
                            _ => {
                                let response: HttpResponse = StaticPageHandler::handle(&req);
                                let _ = response.send_response(stream);
                            },
                        }
                    }
                }
            },
            httprequest::Method::Post => {
                let response: HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = response.send_response(stream);
            },
            httprequest::Method::Uninitialized => {
                let response: HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = response.send_response(stream);
            },
        }
    }
}