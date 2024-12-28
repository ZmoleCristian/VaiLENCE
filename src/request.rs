use openssl::ssl::{SslConnector, SslMethod};
use std::io::{Read, Write};
use std::net::TcpStream;

pub fn build_request(api_key: &str, input_chunk: &str) -> String {
    let request_body = format!(r#"{{"model": "omni-moderation-latest", "input": [{}]}}"#, input_chunk);

    format!(
        "\
            POST /v1/moderations HTTP/1.1\r\n\
            Host: api.openai.com\r\n\
            Authorization: Bearer {}\r\n\
            Content-Type: application/json\r\n\
            Content-Length: {}\r\n\
            Connection: close\r\n\
            \r\n\
            {}",
        api_key,
        request_body.len(),
        request_body
    )
}

pub fn send_request(request: String) -> Vec<u8> {
    let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();
    let mut stream = connector.connect("api.openai.com", TcpStream::connect("api.openai.com:443").unwrap()).unwrap();
    stream.write_all(request.as_bytes()).unwrap();
    stream.flush().unwrap();

    let mut response = Vec::new();
    stream.read_to_end(&mut response).unwrap();
    response
}
