use std::net::TcpStream;

pub struct Response {
    pub status_code: u16,               //TODO
    pub headers: Vec<(String, String)>, //TODO
    pub body: Option<String>,           //TODO
    pub stream: TcpStream,              //TODO
}
