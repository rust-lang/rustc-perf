use collector::api::collected;
use futures::stream::Stream;
use futures::sync::mpsc::{unbounded as unbounded_channel, UnboundedReceiver, UnboundedSender};
use log::warn;
use std::env;
use std::sync::Arc;
use std::thread::{self, JoinHandle};

lazy_static::lazy_static! {
    static ref BG_THREAD: Arc<(UnboundedSender<Option<collected::Request>>, JoinHandle<()>)> =
        start_bg_thread();
}

fn start_bg_thread() -> Arc<(UnboundedSender<Option<collected::Request>>, JoinHandle<()>)> {
    let (sender, receiver): (_, UnboundedReceiver<Option<collected::Request>>) =
        unbounded_channel();
    let sender_thread = sender.clone();
    let handle = thread::spawn(move || {
        for request in receiver.wait() {
            let request = request.expect("result is Ok");
            if request.is_none() {
                // termination requested
                break;
            }
            let request = request.unwrap();
            let ret = (|| {
                let client = reqwest::ClientBuilder::new().build()?;
                let resp = client
                    .post(&format!(
                        "{}/perf/collected",
                        env::var("SITE_URL").expect("SITE_URL defined")
                    ))
                    .bearer_auth(env::var("PERF_SECRET_KEY").unwrap())
                    .json(&request)
                    .send()?;
                if !resp.status().is_success() {
                    bail!("ping home failed: {:?}", resp);
                }
                Ok(())
            })();
            match ret {
                Ok(()) => {}
                Err(err) => {
                    warn!("call home failed: {:?}", err);
                    thread::sleep(std::time::Duration::from_secs(10));
                    sender_thread
                        .unbounded_send(Some(request))
                        .expect("can send on thread channel");
                }
            }
        }
    });
    Arc::new((sender, handle))
}

pub fn send_home(d: collected::Request) {
    BG_THREAD.0.unbounded_send(Some(d)).unwrap();
}

pub fn shut_down() {
    BG_THREAD.0.unbounded_send(None).unwrap();
}
