use std::net::TcpStream;

pub struct Response {
    pub status_code: u16,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
    pub stream: TcpStream,
}
