#[macro_export]
macro_rules! route {
    ($router:ident, $method:expr, $path:literal, $handler:expr) => {{
        fn wrapper<'a>(req: rusty_http::Request<'a>) -> $crate::HandlerResult<'a> {
            Box::pin(async move { $handler(req).await })
        }

        $router.register($method, $path, wrapper)
    }};

    ($router:ident, $method:expr, $path:literal, |$req:ident| $body:block) => {
        $router.register($method, $path, |$req| Box::pin(async move { $body }))
    };
}

#[macro_export]
macro_rules! routes {
    ($router:ident, { $($method:ident $path:literal => $handler:expr),* $(,)? }) => {
        $($crate::$method!($router, $path, $handler);)*
    };
}

#[macro_export]
macro_rules! get { ($r:ident, $p:literal, $($t:tt)*) => { $crate::route!($r, rusty_http::HttpMethod::GET, $p, $($t)*) } }

#[macro_export]
macro_rules! post { ($r:ident, $p:literal, $($t:tt)*) => { $crate::route!($r, rusty_http::HttpMethod::POST, $p, $($t)*) } }

#[macro_export]
macro_rules! put { ($r:ident, $p:literal, $($t:tt)*) => { $crate::route!($r, rusty_http::HttpMethod::PUT, $p, $($t)*) } }

#[macro_export]
macro_rules! delete { ($r:ident, $p:literal, $($t:tt)*) => { $crate::route!($r, rusty_http::HttpMethod::DELETE, $p, $($t)*) } }

#[macro_export]
macro_rules! patch { ($r:ident, $p:literal, $($t:tt)*) => { $crate::route!($r, rusty_http::HttpMethod::PATCH, $p, $($t)*) } }

#[macro_export]
macro_rules! head { ($r:ident, $p:literal, $($t:tt)*) => { $crate::route!($r, rusty_http::HttpMethod::HEAD, $p, $($t)*) } }

#[macro_export]
macro_rules! options { ($r:ident, $p:literal, $($t:tt)*) => { $crate::route!($r, rusty_http::HttpMethod::OPTIONS, $p, $($t)*) } }

#[macro_export]
macro_rules! trace { ($r:ident, $p:literal, $($t:tt)*) => { $crate::route!($r, rusty_http::HttpMethod::TRACE, $p, $($t)*) } }
