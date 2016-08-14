//! Boilerplate for parsing and responding to both GET and POST requests.

use std::ops::Deref;
use std::io::Read;

use serde;
use serde_json::{self, Value};
use iron::prelude::*;
use iron::status;
use persistent::State;

use load::InputData;
use errors::*;

use iron::headers::{ContentType, AccessControlAllowOrigin};
use iron::mime::{Mime, TopLevel, SubLevel};
use iron::modifiers::Header;

pub fn respond(res: Result<Value>) -> IronResult<Response> {
    let mut resp = match res {
        Ok(json) => {
            let mut resp = Response::with((status::Ok, serde_json::to_string(&json).unwrap()));
            resp.set_mut(Header(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![]))));
            resp
        },
        Err(err) => {
            // TODO: Print to stderr
            println!("An error occurred: {:?}", err);
            Response::with((status::InternalServerError, err.to_string()))
        }
    };
    resp.set_mut(Header(AccessControlAllowOrigin::Any));
    Ok(resp)
}

pub trait PostHandler: Sized {
    fn handle(_body: Self, _data: &InputData) -> Result<Value>;

    fn handler(req: &mut Request) -> IronResult<Response>
        where Self: serde::Deserialize {

        let rwlock = req.get::<State<InputData>>().unwrap();
        let data = rwlock.read().unwrap();

        let mut buf = String::new();
        let res = match req.body.read_to_string(&mut buf).unwrap() {
            0 => Err("POST handler with 0 length body.".into()),
            _ => Self::handle(serde_json::from_str(&buf).unwrap(), data.deref())
        };

        respond(res)
    }
}

pub trait GetHandler: Sized {
    fn handle(_data: &InputData) -> Result<Value>;

    fn handler(req: &mut Request) -> IronResult<Response> {
        let rwlock = req.get::<State<InputData>>().unwrap();
        let data = rwlock.read().unwrap();

        respond(Self::handle(data.deref()))
    }
}
