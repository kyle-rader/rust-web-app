mod routes_login;
mod status;

pub fn get_routes() -> axum::Router {
    routes_login::get_routes().merge(status::get_routes())
}
