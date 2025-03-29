use super::HttpMethod;

pub struct Request {
    method: HttpMethod,
    path: String,
    headers: Option<String>, //TODO
    body: Option<String>,    //TODO
}
