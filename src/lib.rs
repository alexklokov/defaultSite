extern crate server;
use server::router::Router;

#[no_mangle]
pub extern "C" fn site() -> Router {
    Router::new()
}
