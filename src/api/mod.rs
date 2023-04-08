use std::path::Path;

use crate::model::io::ApiError;
use rocket::fs::{relative, FileServer};
use rocket::{
    fairing::{Fairing, Info, Kind},
    fs::NamedFile,
    http::Header,
    serde::json::Json,
    Catcher, Request, Response, Route,
};

pub mod auth;
pub mod catalog;
pub mod food_logging;

pub fn get_all_catchers() -> Vec<Catcher> {
    catchers![
        fatal,
        not_authenticated,
        not_authorized,
        not_found,
        bad_request
    ]
}

pub fn get_all_static_routes() -> Vec<Route> {
    let index_route: Vec<Route> = routes![serve_index];
    let mut all_asset_routes: Vec<Route> = FileServer::from(relative!("static/web")).into();
    all_asset_routes.extend(index_route.into_iter());
    return all_asset_routes;
}

#[catch(500)]
pub fn fatal() -> Json<ApiError> {
    Json(ApiError {
        status_code: 500,
        error: "Not your fault. We will look into it. Sorry!".to_owned(),
    })
}

#[catch(401)]
pub fn not_authenticated() -> Json<ApiError> {
    Json(ApiError {
        status_code: 401,
        error: "The request is not authenticated.".to_owned(),
    })
}

#[catch(403)]
pub fn not_authorized() -> Json<ApiError> {
    Json(ApiError {
        status_code: 403,
        error: "The request is not authorized to perform this action.".to_owned(),
    })
}

#[catch(404)]
pub fn not_found() -> Json<ApiError> {
    Json(ApiError {
        status_code: 404,
        error: "The resource you are looking for does not exist.".to_owned(),
    })
}

#[catch(400)]
pub fn bad_request() -> Json<ApiError> {
    Json(ApiError {
        status_code: 400,
        error: "This is a bad request. Check the API documentation.".to_owned(),
    })
}

pub struct CorsFairing;

#[rocket::async_trait]
impl Fairing for CorsFairing {
    fn info(&self) -> Info {
        Info {
            name: "CORS Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, res: &mut Response<'r>) {
        res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
    }
}

pub struct NoCacheFairing;

#[rocket::async_trait]
impl Fairing for NoCacheFairing {
    fn info(&self) -> Info {
        Info {
            name: "No-Cache Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        let route_name = req.route().unwrap().clone().name.unwrap().to_string();
        if route_name == "get_token" {
            res.set_raw_header("Cache-Control", "no-store");
        }
    }
}

#[get("/")]
async fn serve_index() -> Option<NamedFile> {
    let path = Path::new("static/web/index.html");
    NamedFile::open(path).await.ok()
}
