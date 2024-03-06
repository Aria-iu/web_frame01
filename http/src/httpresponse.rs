#![allow(unused)]
use std::{collections::HashMap, fmt::format, io::{self, Write}};

#[derive(Debug,PartialEq,Clone)]
pub struct HttpResponse<'a>{
    version: &'a str,
    status_code: &'a str,
    status_context: &'a str,
    headers: Option<HashMap<&'a str,&'a str>>,
    body: Option<String>
}

impl<'a> Default for HttpResponse<'a>{
    fn default() -> Self {
        Self {  version: "HTTP/1.1".into(), 
                status_code: "200".into(), 
                status_context: "OK".into(), 
                headers: None, 
                body: None, 
        }
    }
}

impl<'a> HttpResponse<'a>{
    pub fn new(status_code:&'a str,headers: Option<HashMap<&'a str, &'a str>>,body: Option<String>) -> HttpResponse<'a>{
        let mut response:HttpResponse<'a> = HttpResponse::default();
        if status_code!="200"{
            response.status_code = status_code.into();
        };
        response.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };
        response.status_context = match response.status_code {
            "200" => "OK".into(),
            "400" => "Bad Request".into(),
            "404" => "Not Found".into(),
            "500" => "Internal Server Error".into(),
            _ => "NOT FOUND".into(),
        };
        response.body = body;
        response
    }

    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<(),io::Error>{
        let res = self.clone();
        let response_string = String::from(res);
        let _ = write!(write_stream,"{}",response_string);
        Ok(())
    }

    fn version(&self) -> &str{
        self.version
    }
    fn status_code(&self) -> &str{
        self.status_code
    }
    fn status_context(&self) -> &str{
        self.status_context
    }
    fn headers(&self) -> String{
        let map = self.headers.clone().unwrap();
        let mut header_string = "".to_string();
        for (k,v) in map.iter(){
            header_string = format!("{}{}:{}\r\n",header_string,k,v);
        }
        header_string
    }
    pub fn body(&self) -> &str{
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }
}

impl<'a> From<HttpResponse<'a>> for String{
    fn from(value: HttpResponse<'a>) -> Self {
        let res = value.clone();
        format!("{} {} {}\r\n{}Content-Length:{}\r\n\r\n{}", &res.version() , &res.status_code(), &res.status_context() ,&res.headers(),&value.body.unwrap().len(),&res.body())
    }
}





#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_response_struct_creation_200() {
        let response_actual = HttpResponse::new(
            "200",
            None,
            Some("Item was shipped on 21st Dec 2020".into()),
        );
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_context: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }, 
            body: Some("Item was shipped on 21st Dec 2020".into()),
        };
        assert_eq!(response_actual, response_expected);
    }
}
