extern crate futures;
extern crate hyper;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate time;
extern crate tokio_core;
extern crate tokio_service;
extern crate tokio_tungstenite;
extern crate tungstenite;
extern crate uuid;

#[macro_use]
extern crate error_chain;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::str;
use std::time::Duration;

use futures::future::{err, result, ok, Loop, loop_fn, Either};
use futures::unsync::mpsc;
use futures::{Stream, Future, IntoFuture, Sink, Async, Poll};
use hyper::Method;
use hyper::server::Http;
use tokio_core::net::TcpListener;
use tokio_core::reactor::{Core, Timeout, Handle};
use tokio_service::Service;
use tokio_tungstenite::accept_async;
use tungstenite::Message;
use uuid::Uuid;

use errors::*;

mod errors {
    use std::io;

    use tungstenite;
    use serde_json;
    use futures::Future;

    error_chain! {
		foreign_links {
			Ws(tungstenite::Error);
			Io(io::Error);
            Json(serde_json::Error);
		}
    }

    pub type MyFuture<T> = Box<Future<Item = T, Error = Error>>;
}

#[derive(Deserialize)]
#[serde(tag = "messageType", rename_all = "lowercase")]
enum ClientMessage {
    Hello {
        uaid: Option<Uuid>,
        #[serde(rename = "channelIDs", skip_serializing_if = "Option::is_none")]
        channel_ids: Option<Vec<Uuid>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        use_webpush: Option<bool>,
    },

    Register {
        #[serde(rename = "channelID")]
        channel_id: Uuid,
    },

    Unregister {
        #[serde(rename = "channelID")]
        channel_id: Uuid,
    },
}

#[derive(Serialize)]
#[serde(tag = "messageType", rename_all = "lowercase")]
enum ServerMessage {
    Hello {
        uaid: Uuid,
        status: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        use_webpush: Option<bool>,
    },

    Register {
        #[serde(rename = "channelID")]
        channel_id: Uuid,
        status: u32,
        #[serde(rename = "pushEndpoint")]
        push_endpoint: String,
    },

    Unregister {
        #[serde(rename = "channelID")]
        channel_id: Uuid,
        status: u32,
    },

    Notification(Notification),
}

#[derive(Serialize)]
#[serde(untagged)]
enum Notification {
    Simple {
        updates: Vec<Update>,
    },

    // WebPush {
    //     #[serde(rename = "channelID")]
    //     channel_id: Uuid,
    //     version: String,
    // },
}


#[derive(Serialize)]
struct Update {
    #[serde(rename = "channelID")]
    channel_id: Uuid,
    version: u64,
}

struct Context {
    channels: RefCell<HashMap<Uuid, Channel>>,

    // maps to connected channels
    uaids: RefCell<HashMap<Uuid, Client>>,
}

struct Channel {
    uaid: Uuid,
    current_version: u64,
}

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let ws_listener = TcpListener::bind(&"127.0.0.1:8080".parse().unwrap(), &handle)
        .unwrap();

    let cx = Rc::new(Context {
        channels: RefCell::new(HashMap::new()),
        uaids: RefCell::new(HashMap::new()),
    });

    let ws_srv = ws_listener.incoming().for_each(|(socket, addr)| {
        // Perform the websocket handshake followed by the webpush handshake.
        // Time out this whole process in case it takes too long.
        let ws = accept_async(socket).from_err();
        let client = ws.and_then(move |ws| {
            let ws = ws.from_err()
                .and_then(parse)
                .with(serialize);
            Client::handshake(ws)
        });
        let handshake_timeout = Duration::new(30, 0);
        let client = timeout(client, handshake_timeout, &handle);

        // Now that we've got a connected client, start processing messages.
        // This'll register our new `Client` with our global `Context` and then
        // it'll also unregister everything when our client goes away.
        //
        // TODO: shouldn't unreigster everything I think?
        let handle2 = handle.clone();
        let cx2 = cx.clone();
        let work  = client.and_then(move |(ws, client, rx)| {
            let uaid = client.uaid;
            client.process(ws, cx2.clone(), rx, &handle2).then(move |res| {
                let mut uaids = cx2.uaids.borrow_mut();
                let mut channels = cx2.channels.borrow_mut();
                for id in uaids.remove(&uaid).expect("uaid not registered").channel_ids {
                    channels.remove(&id).expect("uaid pointed to missing channel");
                }
                return res
            })
        });

        // Spawn this client to happen concurrently with all other clients
        handle.spawn(work.then(move |result| {
            match result {
                Ok(()) => println!("peer {} gone", addr),
                Err(e) => {
                    println!("{} error: {}", addr, e);
                    for e in e.iter().skip(1) {
                        println!("\t{}", e);
                    }
                }
            }
            Ok(())
        }));
        Ok(())
    });

    let push_listener = TcpListener::bind(&"127.0.0.1:8081".parse().unwrap(), &handle)
        .unwrap();
    let proto = Http::new();
    let cx = cx.clone();
    let push_srv = push_listener.incoming().for_each(|(socket, addr)| {
        proto.bind_connection(&handle, socket, addr, Push(cx.clone()));
        Ok(())
    });

    core.run(ws_srv.join(push_srv)).ok().unwrap();
}

fn parse(message: Message) -> Result<ClientMessage> {
    match message {
        Message::Text(s) => {
            println!("parse {}", s);
            serde_json::from_str(&s).chain_err(|| "invalid json text")
        }
        Message::Binary(slice) => {
            serde_json::from_slice(&slice).chain_err(|| "invalid json bytes")
        }
    }
}

fn serialize(message: ServerMessage) -> Result<Message> {
    let string = serde_json::to_string(&message).chain_err(|| "failed to serialize")?;
    Ok(Message::Text(string))
}

struct Client {
    uaid: Uuid,
    use_webpush: bool,
    channel_ids: Vec<Uuid>,
    tx: mpsc::UnboundedSender<Notification>,
}

impl Client {
    fn handshake<T>(ws: T) -> MyFuture<(T, Client, mpsc::UnboundedReceiver<Notification>)>
        where T: Sink<SinkItem = ServerMessage, SinkError = Error> +
                    Stream<Item = ClientMessage, Error = Error> +
                    'static,
    {
        Box::new(ws.into_future().then(move |res| -> MyFuture<_> {
            let (msg, ws) = match res {
                Ok(pair) => pair,
                Err((e, _rx)) => {
                    return Box::new(result(Err(e).chain_err(|| "recv error")))
                }
            };
            let msg = match msg {
                Some(msg) => msg,
                None => return Box::new(err("terminated before handshake".into())),
            };

            let (tx, rx) = mpsc::unbounded();
            let client = match msg {
                ClientMessage::Hello { uaid, channel_ids, use_webpush } => {
                    Client {
                        uaid: uaid.unwrap_or_else(Uuid::new_v4),
                        use_webpush: use_webpush.unwrap_or(false),
                        channel_ids: channel_ids.unwrap_or(Vec::new()),
                        tx: tx,
                    }
                }
                _ => return Box::new(err("non-hello message before handshake".into())),
            };
            let response = ServerMessage::Hello {
                uaid: client.uaid,
                status: 200,
                use_webpush: Some(client.use_webpush),
            };
            Box::new(ws.send(response).map(|ws| {
                (ws, client, rx)
            }))
        }))
    }

    fn process<T>(self,
                  ws: T,
                  cx: Rc<Context>,
                  rx: mpsc::UnboundedReceiver<Notification>,
                  handle: &Handle) -> MyFuture<()>
        where T: Sink<SinkItem = ServerMessage, SinkError = Error> +
                    Stream<Item = ClientMessage, Error = Error> +
                    'static,
    {
        let uaid = self.uaid;

        // TODO: handle already-present channels
        for id in self.channel_ids.iter() {
            assert!(cx.channels.borrow_mut().insert(*id, Channel {
                uaid: self.uaid,
                current_version: 0,
            }).is_none());
        }

        // TODO: if this is a duplicate we should respond by requesting the
        //       client selects a new uaid
        assert!(cx.uaids.borrow_mut().insert(self.uaid, self).is_none());

        let handle = handle.clone();
        let rx = rx.map_err(|_| panic!());
        Box::new(loop_fn((ws, rx), move |(ws, rx)| {
            let handle = handle.clone();
            let cx = cx.clone();
            StreamNext::new(ws, rx).then(move |res| -> MyFuture<_> {
                let (msg, ws, rx) = match res {
                    Ok(None) => return Box::new(ok(Loop::Break(()))),
                    Ok(Some(res)) => res,
                    Err(e) => {
                        return Box::new(result(Err(e).chain_err(|| "recv error")))
                    }
                };

                let response = match msg {
                    Either::A(ClientMessage::Hello { .. }) => {
                        return Box::new(err("double hello received".into()))
                    }
                    Either::A(ClientMessage::Register { channel_id }) => {
                        let mut uaids = cx.uaids.borrow_mut();
                        let mut channels = cx.channels.borrow_mut();
                        let client = uaids.get_mut(&uaid).unwrap();
                        let status = if channels.contains_key(&channel_id) {
                            409
                        } else {
                            channels.insert(channel_id, Channel {
                                uaid: uaid,
                                current_version: 0,
                            });
                            client.channel_ids.push(channel_id);
                            200
                        };
                        ServerMessage::Register {
                            status: status,
                            channel_id: channel_id,
                            push_endpoint: format!("http://localhost:8081/{}",
                                                   channel_id),
                        }
                    }
                    Either::A(ClientMessage::Unregister { channel_id }) => {
                        ServerMessage::Unregister {
                            status: 200,
                            channel_id: channel_id,
                        }
                    }
                    Either::B(n) => ServerMessage::Notification(n),
                };
                let ws = timeout(ws.send(response),
                                 Duration::new(600, 0),
                                 &handle);
                Box::new(ws.map(|ws| Loop::Continue((ws, rx))))
            })
        }))
    }
}

fn timeout<F>(f: F, dur: Duration, handle: &Handle) -> MyFuture<F::Item>
    where F: Future<Error = Error> + 'static,
{

    let timeout = Timeout::new(dur, handle).into_future().flatten();
    Box::new(f.select2(timeout).then(|res| {
        match res {
            Ok(Either::A((item, _timeout))) => Ok(item),
            Ok(Either::B(((), _item))) => Err("accept timed out".into()),
            Err(Either::A((e, _timeout))) => Err(e),
            Err(Either::B((e, _item))) => Err(e).chain_err(|| "timeout failure"),
        }
    }))
}


struct Push(Rc<Context>);

impl Service for Push {
    type Request = hyper::Request;
    type Response = hyper::Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = hyper::Response, Error = hyper::Error>>;

    fn call(&self, req: hyper::Request) -> Self::Future {
        use hyper::header::ContentType;

        if *req.method() != Method::Put && *req.method() != Method::Post {
            println!("not a PUT: {}", req.method());
            return Box::new(err(hyper::Error::Method))
        }
        if req.uri().path().len() == 0 {
            println!("empty uri path");
            return Box::new(err(hyper::Error::Incomplete))
        }
        let channel_id = match req.uri().path()[1..].parse::<Uuid>() {
            Ok(id) => id,
            Err(_) => {
                println!("uri not uuid: {}", req.uri().path());
                return Box::new(err(hyper::Error::Status))
            }
        };
        let form_encoded = match req.headers().get::<ContentType>() {
            Some(header) => **header == "application/x-www-form-urlencoded",
            None => false,
        };

        let body = req.body().concat2();
        let cx = self.0.clone();
        Box::new(body.and_then(move |body| {
            let version = if body.len() == 0 {
                time::now_utc().to_timespec().sec as u64
            } else {
                if !form_encoded {
                    println!("bad content type");
                    return Err(hyper::Error::Incomplete)
                }
                let header = b"version=";
                if !body.starts_with(header) {
                    println!("bad body");
                    return Err(hyper::Error::Incomplete)
                }
                let vers = str::from_utf8(&body[header.len()..]).ok()
                    .and_then(|s| s.parse::<u64>().ok());
                match vers {
                    Some(vers) => vers,
                    None => {
                        println!("failed to parse version");
                        return Err(hyper::Error::Incomplete)
                    }
                }
            };
            let mut channels = cx.channels.borrow_mut();
            let channel = match channels.get_mut(&channel_id) {
                Some(channel) => channel,
                None => {
                    println!("no channel found for channel id specified");
                    return Err(hyper::Error::Incomplete)
                }
            };
            channel.current_version = version;

            let mut uaids = cx.uaids.borrow_mut();
            let uaid = uaids.get_mut(&channel.uaid).unwrap();
            (&uaid.tx).send(Notification::Simple {
                updates: vec![Update {
                    channel_id: channel_id,
                    version: version,
                }],
            }).unwrap();

            Ok(hyper::Response::new())
        }))
    }
}

struct StreamNext<S1, S2> {
    left: Option<S1>,
    right: Option<S2>,
}

impl<S1, S2> StreamNext<S1, S2>
    where S1: Stream,
          S2: Stream<Error = S1::Error>,
{
    fn new(s1: S1, s2: S2) -> StreamNext<S1, S2> {
        StreamNext {
            left: Some(s1),
            right: Some(s2),
        }
    }
}

impl<S1, S2> Future for StreamNext<S1, S2>
    where S1: Stream,
          S2: Stream<Error = S1::Error>,
{
    type Item = Option<(Either<S1::Item, S2::Item>, S1, S2)>;
    type Error = S1::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let item = {
            let left = self.left.as_mut().unwrap();
            let right = self.right.as_mut().unwrap();
            match left.poll()? {
                Async::Ready(None) => return Ok(Async::Ready(None)),
                Async::Ready(Some(item)) => Either::A(item),
                Async::NotReady => {
                    match right.poll()? {
                        Async::Ready(None) => return Ok(Async::Ready(None)),
                        Async::Ready(Some(item)) => Either::B(item),
                        Async::NotReady => return Ok(Async::NotReady),
                    }
                }
            }
        };
        Ok(Async::Ready(Some((item,
                              self.left.take().unwrap(),
                              self.right.take().unwrap()))))
    }
}
