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
