mod request;

fn main() -> Result<(), ureq::Error> {
    let json: serde_json::Value = ureq::json!({
        "name": "martin",
        "rust": true,
        "nested": {
            "id": 1,
            "name": "Homer"
        }
    });

    let response = request::do_request(
        "https://httpbin.org/anything".into(),
        request::Method::POST,
        json,
    )?;

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
