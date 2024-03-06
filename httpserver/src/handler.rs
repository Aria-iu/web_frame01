use std::{collections::HashMap, env, fs};

use http::{httprequest::{self, HttpRequest}, httpresponse::HttpResponse};
use serde::{Deserialize, Serialize};

pub trait Handler {
    /// This method has to be implemented for any other user data type to implement the trait.
    fn handle(req: &HttpRequest) -> HttpResponse;

    /// This method loads a file (non-JSON) from the public directory in the httpserver root folder. 
    /// The implementation is already provided as part of the trait definition.
    /// default method impl
    fn load_file(filename: &str) -> Option<String>{
        let default_path = format!("{}/public",env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}",public_path,filename);
        let contents = fs::read_to_string(full_path);
        contents.ok()
    }
}

pub struct PageNotFoundHandler;
pub struct WebServiceHandler;
pub struct StaticPageHandler;

impl Handler for PageNotFoundHandler{
    fn handle(_req: &HttpRequest) -> HttpResponse {
        HttpResponse::new("404", None, Self::load_file("404.html"))
    }
}
impl Handler for WebServiceHandler{
    fn handle(req: &HttpRequest) -> HttpResponse {
        let httprequest::Resource::Path(s) = &req.resource;
        let route:Vec<&str> = s.split("/").collect();
        match route[2] {
            "shipping" if route.len() > 2 && route[3] == "orders" =>{
                let body = Some(serde_json::to_string(&Self::load_json()).unwrap());
                let mut headers: HashMap<&str, &str> = HashMap::new();
                headers.insert("Content-Type", "application/json");
                HttpResponse::new("200", Some(headers), body)
            },
            _ => {
                HttpResponse::new("404", None, Self::load_file("404.html"))
            }
        }
    }
}
impl Handler for StaticPageHandler{
    fn handle(req: &HttpRequest) -> HttpResponse {
        //let resource = &req.resource;
        let http::httprequest::Resource::Path(s) = &req.resource;
        let route:Vec<&str> = s.split("/").collect();
        match route[1]{
            "" => {
                HttpResponse::new("200", None, Self::load_file("index.html"))
            },
            "health" => {
                HttpResponse::new("200", None, Self::load_file("health.html"))
            },
            path => {
                match Self::load_file(path) {
                    Some(contents) => {

                        let mut map = HashMap::new();
                        if path.ends_with("css"){map.insert("Content-Type", "text/css");}
                        else if path.ends_with("js"){map.insert("Content-Type", "text/javascript");}
                        else {map.insert("Content-Type", "text/html");}

                        HttpResponse::new("200", Some(map), Some(contents))
                    },
                    None => {
                        HttpResponse::new("404", None, Self::load_file("404.html"))
                    },
                }
            },
        }

    }
}




// transport in the wire
#[derive(Serialize,Deserialize)]
pub struct OrderStatus{
    order_id: i32,
    order_date: String,
    order_status: String,
}

impl WebServiceHandler{
    fn load_json() -> Vec<OrderStatus>{
        let default_path = format!("{}/data",env!("CARGO_MANIFEST_DIR"));
        let data_path = env::var("DATA_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", data_path, "orders.json");
        let json_contents = fs::read_to_string(full_path);
        let orders = serde_json::from_str(json_contents.unwrap().as_str()).unwrap();
        orders
    }
}























#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_404_ok(){
        let req: HttpRequest = "GET /greeting HTTP/1.1\r\nHost:localhost:3000\r\nUser-Agent:curl/7.64.1\r\nAccept:*/*\r\n\r\n".to_string().into();
        let resp = PageNotFoundHandler::handle(&req);
        println!("{:?}",resp);
    }
}
