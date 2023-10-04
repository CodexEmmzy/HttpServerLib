use std::collections::HashMap;
use std::io::{Result, Write};

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self { 
            version: "HTTP/1.1", 
            status_code: "200".into(), 
            status_text: "OK".into(), 
            headers: None, 
            body: None 
        }
    }
}

impl<'a> HttpResponse<'a> {

    // The new() method starts by constructing a struct with default parameters. The
    // values passed as parameters are then evaluated and incorporated into the
    // struct.

    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> HttpResponse<'a> {

        let mut response: HttpResponse<'a> = HttpResponse::default();
        if status_code != "200" {
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

        response.status_text = match response.status_code {
            "200" => "OK".into(),
            "400" => "Bad Request".into(),
            "404" => "Not Found".into(),
            "500" => "Internal Server Error".into(),
            _ => "Not Found".into(),
        };

        response.body = body;

        response
    }


    
    // The send_response() method is used to convert the HttpResponse struct into a
    // String, and transmit it over the TCP connection. This can be added within the
    // impl block, after the new() method in httpresponse.rs.

    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
        let res = self.clone();
        let response_string: String = String::from(res);
        let _ = write!(write_stream, "{}", response_string);

        Ok(())
    }
}

// Code to serialize Rust struct into HTTP Response message
impl<'a> From<HttpResponse<'a>> for String {
    fn from(res: HttpResponse) -> String {
        let res1 = res.clone();
        format!(
            "{} {} {}\r\n{:?}Content-Length: {}\r\n\r\n{}",
            &res1.version,
            &res1.status_code,
            &res1.status_text,
            &res1.headers.unwrap(),
            &res.body.unwrap().len(),
            &res1.body.unwrap(),
        )
    }
}

#[cfg(test)]
mod tests {
    use  super::*;
    #[test]
    // Test script for HTTP success (200) message
    fn test_response_struct_creation_200() {
        let response_actual = HttpResponse::new(
            "200",
            None,
            Some("Item was shipped on 21st Dec 2020".into()),
        );

        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("Item was shipped on 21st Dec 2020".into()),
        };

        assert_eq!(response_actual, response_expected);
    }

    // Test script for 404 message
    #[test]
    fn test_response_struct_creation_404() {
        let response_actual = HttpResponse::new(
            "404",
            None,
            Some("Item was shipped on 21st Dec 2020".into()),
        );

        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("Item was shipped on 21st Dec 2020".into()),
        };

        assert_eq!(response_actual, response_expected);

    }
    
    // Test script to check for well-formed HTTP response message
    #[test]
    fn test_http_response_creation() {
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("Item was shipped on 21st Dec 2020".into()),
        };

        let http_string: String = response_expected.into();
        let response_actual = r"HTTP/1.1 404 Not Found\r\nContent-Type:text/html\r\nContent-Length:33\r\n\hh";
        assert_eq!(http_string, response_actual);
    }

}