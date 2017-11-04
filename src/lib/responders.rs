use rocket::Response;
use rocket::request::Request;
use rocket::response::{Responder};
use rocket::http::Status;

pub struct Cors<R>(pub R);

impl<'r, R: Responder<'r>> Responder<'r> for Cors<R> {
    #[inline(always)]
    fn respond_to(self, req: &Request) -> Result<Response<'r>, Status> {
        Response::build()
            .merge(self.0.respond_to(req)?)
            .raw_header("Access-Control-Allow-Origin", "*")
            .ok()
    }
}


pub struct Cached<R>(pub R);

impl<'r, R: Responder<'r>> Responder<'r> for Cached<R> {
    #[inline(always)]
    fn respond_to(self, req: &Request) -> Result<Response<'r>, Status> {
        Response::build()
            .merge(self.0.respond_to(req)?)
            .raw_header("Cache-Control", "max-age=86400") // 1 day
            .ok()
    }
}
