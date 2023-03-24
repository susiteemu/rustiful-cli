use ureq::Response;

enum Method {
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

fn do_request(
    path: String,
    method: Method,
    json: serde_json::Value,
) -> Result<Response, ureq::Error> {
    return ureq::request(&method.value(), &path)
        .set("Content-Type", "application/json")
        .send_json(&json);
}

fn main() -> Result<(), ureq::Error> {
    let json: serde_json::Value = ureq::json!({
        "name": "martin",
        "rust": true,
        "nested": {
            "id": 1,
            "name": "Homer"
        }
    });

    let response = do_request("https://httpbin.org/anything".into(), Method::POST, json)?;

    println!("Status: {} {}", &response.status(), &response.status_text());

    let header_names = &response.headers_names();
    for header_name in header_names {
        println!(
            "{}: {:?}",
            &header_name,
            &response.header(&header_name).unwrap()
        );
    }

    let body: serde_json::Value = response.into_json()?;
    println!("{}", serde_json::to_string_pretty(&body).unwrap());

    Ok(())
}
