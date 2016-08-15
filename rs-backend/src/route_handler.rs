// Copyright 2016 The rustc-perf Project Developers. See the COPYRIGHT
// file at the top-level directory.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Boilerplate for parsing and responding to both GET and POST requests.

use std::io::Read;

use serde;
use serde_json::{self, Value};
use iron::headers::{ContentType, AccessControlAllowOrigin};
use iron::mime::{Mime, TopLevel, SubLevel};
use iron::modifiers::Header;
use iron::prelude::*;
use iron::status;
use persistent::State;

use load::InputData;
use errors::*;


pub fn unwrap_with_or_internal_err<T, F>(x: Result<T>, f: F) -> Response
    where F: FnOnce(T) -> Response
{
    match x {
        Ok(x) => f(x),
        Err(err) => {
            // TODO: Print to stderr
            println!("An error occurred: {:?}", err);
            let mut response = Response::with((status::InternalServerError, err.to_string()));
            response.set_mut(Header(AccessControlAllowOrigin::Any));
            response
        }
    }
}

pub fn respond(res: Value) -> Response {
    let mut response = Response::with((status::Ok, serde_json::to_string(&res).unwrap()));
    response.set_mut(Header(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![]))));
    response.set_mut(Header(AccessControlAllowOrigin::Any));
    response
}

pub fn handler_post<F, T>(req: &mut Request, handle_data: F) -> IronResult<Response>
    where T: serde::Deserialize,
          F: FnOnce(T, &InputData) -> Value
{
    let rwlock = req.get::<State<InputData>>().unwrap();
    let data = rwlock.read().unwrap();

    let mut buf = String::new();
    let data = match req.body.read_to_string(&mut buf).unwrap() {
        0 => Err("POST handler with 0 length body.".into()),
        _ => Ok(handle_data(serde_json::from_str(&buf).unwrap(), &data)),
    };

    Ok(unwrap_with_or_internal_err(data, respond))
}

pub fn handler_get<F>(req: &mut Request, handle_data: F) -> IronResult<Response>
    where F: FnOnce(&InputData) -> Value
{
    let rwlock = req.get::<State<InputData>>().unwrap();
    let data = rwlock.read().unwrap();

    Ok(respond(handle_data(&data)))
}
