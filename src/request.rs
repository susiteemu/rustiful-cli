use ureq::Response;

pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTIONS,
    HEAD,
}

impl Method {
    fn value(&self) -> &str {
        match *self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::PATCH => "PATCH",
            Method::DELETE => "DELETE",
            Method::OPTIONS => "OPTIONS",
            Method::HEAD => "HEAD",
        }
    }
}

pub fn do_request(
    path: String,
    method: Method,
    json: serde_json::Value,
) -> Result<Response, ureq::Error> {
    return ureq::request(&method.value(), &path)
        .set("Content-Type", "application/json")
        .send_json(&json);
}
