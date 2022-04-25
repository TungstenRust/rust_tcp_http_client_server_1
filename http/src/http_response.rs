use std::collections::HashMap;
use std::io::{Result, Write};

//The Structure of HTTP response
#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>
}
//The Default Trait behaviour for HTTP response
impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1".into(),
            status_code: "200".into(),
            status_text: "OK".into(),
            headers: None,
            body: None,
        }
    }
}


/**new() method for HttpResponse (httpresponse.rs)
The new() method accepts a some parameters ,
 sets the default for the others and returns a HttpResponse struct.
 Add the following code under impl block of HttpResponse struct.
 As this struct has a reference type for one of its members,
 the impl block declaration has to also specify a lifetime parameter
*/
impl<'a> HttpResponse<'a> {
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
    //Convert the HttpResponse struct into a String, and transmit it over the TCP connection
    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
        let resp = self.clone();
        let response_string: String = String::from(resp);
        let _ = write!(write_stream, "{}", response_string);
        Ok(())
    }
    //Getter methods for HttpResponse
    fn version(&self) -> &str {
        self.version
    }
    fn status_code(&self) -> &str {
        self.status_code
    }
    fn status_text(&self) -> &str {
        self.status_text
    }
    fn headers(&self) -> String {
        let map: HashMap<&str, &str> = self.headers.clone().unwrap();
        let mut header_string: String = "".into();
        for (k, v) in map.iter() {
            header_string = format!("{}{}:{}\r\n", header_string, k, v);
        }
        header_string
    }
    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }
}
//Serialize Rust struct into HTTP Response message
impl<'a> From<HttpResponse<'a>> for String {
    fn from(resp: HttpResponse) -> String {
        let resp1 = resp.clone();
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            &resp1.version(),
            &resp1.status_code(),
            &resp1.status_text(),
            &resp1.headers(),
            &resp.body.unwrap().len(),
            &resp1.body()
        )
    }
}

//Test for HTTP success (200) message
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_response_struct_creation_200() {
        let response_actual = HttpResponse::new(
            "200",
            None,
            Some("Item was shipped on 22nd April 2022".into()),
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
            body: Some("Item was shipped on 22nd April 2022".into()),
        };
        assert_eq!(response_actual, response_expected);
    }
    //Test for 404 message
    fn test_response_struct_creation_404() {
        let response_actual = HttpResponse::new(
            "404",
            None,
            Some("Item was shipped on 22nd April 2022".into()),
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
            body: Some("Item was shipped on 22nd April 2022".into()),
        };
        assert_eq!(response_actual, response_expected);
    }
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
            body: Some("Item was shipped on 22nd April 2022".into()),
        };
        let http_string: String = response_expected.into();
        let response_actual = "HTTP/1.1 404 Not Found\r\nContent-Type:text/html\r\nContent-Length: 33\r\n\r\nItem was shipped on 22nd April 2022";
        assert_eq!(http_string, response_actual);
    }
}