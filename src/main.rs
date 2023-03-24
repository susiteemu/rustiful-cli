mod request;
use std::env;

fn main() -> Result<(), ureq::Error> {
    let args: Vec<String> = env::args().collect();

    let url: &str = &args[1];
    let payload: &str = &args[2];

    let json = serde_json::from_str(payload).unwrap();

    let response = request::do_request(url.into(), request::Method::GET, json)?;

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
