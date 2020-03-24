use anyhow::bail;
use collector::api::collected;
use crossbeam_channel::{unbounded, Receiver, Sender};
use log::warn;
use std::env;
use std::sync::RwLock;
use std::thread;

lazy_static::lazy_static! {
    static ref BG_THREAD: RwLock<Option<(Sender<collected::Request>, thread::JoinHandle<()>)>> =
        RwLock::new(Some(start_bg_thread()));
}

fn call_home(client: &reqwest::blocking::Client, request: &collected::Request) -> anyhow::Result<()> {
    let resp = client
        .post(&format!(
            "{}/perf/collected",
            env::var("SITE_URL").expect("SITE_URL defined")
        ))
        .bearer_auth(env::var("PERF_SECRET_KEY").unwrap())
        .json(request)
        .send()?;
    if !resp.status().is_success() {
        bail!("ping home failed: {:?}", resp);
    }
    Ok(())
}

fn start_bg_thread() -> (Sender<collected::Request>, thread::JoinHandle<()>) {
    let (sender, receiver): (_, Receiver<collected::Request>) = unbounded();
    let client = reqwest::blocking::Client::new();
    let handle = thread::spawn(move || {
        for request in receiver {
            while let Err(err) = call_home(&client, &request) {
                warn!("call home failed: {:?}", err);
                thread::sleep(std::time::Duration::from_secs(10));
            }
        }
    });
    (sender, handle)
}

pub fn send_home(d: collected::Request) {
    let sender = BG_THREAD.read().unwrap_or_else(|e| e.into_inner());
    let sender = sender.as_ref().expect("worker not shutdown");
    sender.0.send(d).unwrap();
}

pub fn shut_down() {
    let mut sender = BG_THREAD.write().unwrap_or_else(|e| e.into_inner());
    let (sender, handle) = sender.take().expect("not yet shutdown");
    // close the channel
    std::mem::drop(sender);
    // wait for the thread to finish
    handle.join().unwrap();
}
