#![allow(unused)]

use core::str;
use std::collections::HashMap;

// http versions
#[derive(Debug,PartialEq)]
pub enum Version{
    V1_1,
    V2_0,
    Uninitialized,
}

// enum to specify the allowed values for HTTP method
#[derive(Debug,PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Version{
    fn from(value: &str) -> Self {
        match value {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::Uninitialized,
        }
    }
}

impl From<&str> for Method{
    fn from(value: &str) -> Self {
        match value{
            "GET"  => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}


// data struct for http request
#[derive(Debug)]
pub struct HttpRequest{
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String,String>,
    pub msg_body: String,
}

#[derive(Debug,PartialEq)]
pub enum Resource{
    Path(String),
}

impl From<String> for HttpRequest{
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::Uninitialized;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_header = HashMap::new();
        let mut parsed_msg_body = "".to_string();

        for line in req.lines(){
            if line.contains("HTTP"){
                let (method,resouece,version) = process_request_line(line);
                parsed_method = method;
                parsed_resource = resouece;
                parsed_version = version;
            }else if line.contains(":"){
                let (key,value) = process_header_line(line);
                parsed_header.insert(key, value);
            }else if line.len() == 0{

            }else{
                parsed_msg_body = line.to_string();
            }
        }

        HttpRequest{
            method: parsed_method,
            version: parsed_version,
            headers: parsed_header,
            resource: parsed_resource,
            msg_body: parsed_msg_body,
        }
    }

    
}

fn process_request_line(s: &str) -> (Method,Resource,Version){
    let mut words = s.split_whitespace();
    // some cheat
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();
    (method.into(),Resource::Path(resource.to_string()),version.into())
}
fn process_header_line(s: &str) -> (String,String){
    let mut header_items = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");
    if let Some(k) = header_items.next() {
        key = k.to_string();
    }
    if let Some(v) = header_items.next() {
        value = v.to_string()
    }
    (key,value)
}












#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);
    }

    #[test]
    fn test_version_into() {
        let m: Version = "HTTP/1.1".into();
        assert_eq!(m, Version::V1_1);
    }

    #[test]
    fn test_read_http() {
        // you can not use ':' in msg_body in this versionl
        let s: String = "GET /greeting HTTP/1.1\r\nHost:localhost:3000\r\nUser-Agent:curl/7.64.1\r\nAccept:*/*\r\n\r\n".to_string();
        let mut headers_expected:HashMap<String, String> = HashMap::new();
        headers_expected.insert("Host".into(), "localhost".into());
        headers_expected.insert("User-Agent".into(), "curl/7.64.1".into());
        headers_expected.insert("Accept".into(), "*/*".into());
        let req : HttpRequest = s.into();
        assert_eq!(Method::Get , req.method);
        assert_eq!(Version::V1_1 , req.version);
        assert_eq!(Resource::Path("/greeting".to_string()) , req.resource);
        assert_eq!(headers_expected , req.headers);
    }
}