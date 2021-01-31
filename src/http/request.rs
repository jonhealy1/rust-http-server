pub mod request {
    use super::method::Method;

    struct Request {
        path: String,
        query_string: Option<String>,
        method: Method,
    }
}