use super::controller::{all,create};
use actix_web::web;

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(all);
    cfg.service(create);
}
