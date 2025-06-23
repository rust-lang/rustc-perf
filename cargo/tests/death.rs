extern crate cargotest;
extern crate libc;

use std::fs;
use std::io::{self, Read};
use std::net::TcpListener;
use std::process::{Stdio, Child};
use std::thread;
use std::time::Duration;

use cargotest::support::project;

#[cfg(unix)]
fn enabled() -> bool {
    true
}

// On Windows suport for these tests is only enabled through the usage of job
// objects. Support for nested job objects, however, was added in recent-ish
// versions of Windows, so this test may not always be able to succeed.
//
// As a result, we try to add ourselves to a job object here
// can succeed or not.
#[cfg(windows)]
fn enabled() -> bool {
    extern crate kernel32;
    extern crate winapi;
    unsafe {
        // If we're not currently in a job, then we can definitely run these
        // tests.
        let me = kernel32::GetCurrentProcess();
        let mut ret = 0;
        let r = kernel32::IsProcessInJob(me, 0 as *mut _, &mut ret);
        assert!(r != 0);
        if ret == winapi::FALSE {
            return true
        }

        // If we are in a job, then we can run these tests if we can be added to
        // a nested job (as we're going to create a nested job no matter what as
        // part of these tests.
        //
        // If we can't be added to a nested job, then these tests will
        // definitely fail, and there's not much we can do about that.
        let job = kernel32::CreateJobObjectW(0 as *mut _, 0 as *const _);
        assert!(!job.is_null());
        let r = kernel32::AssignProcessToJobObject(job, me);
        kernel32::CloseHandle(job);
        r != 0
    }
}

#[test]
fn ctrl_c_kills_everyone() {
    if !enabled() {
        return
    }

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    let p = project("foo")
        .file("Cargo.toml", r#"
            [package]
            name = "foo"
            version = "0.0.1"
            authors = []
            build = "build.rs"
        "#)
        .file("src/lib.rs", "")
        .file("build.rs", &format!(r#"
            use std::net::TcpStream;
            use std::io::Read;

            fn main() {{
                let mut socket = TcpStream::connect("{}").unwrap();
                let _ = socket.read(&mut [0; 10]);
                panic!("that read should never return");
            }}
        "#, addr))
        .build();

    let mut cargo = p.cargo("build").build_command();
    cargo.stdin(Stdio::piped())
         .stdout(Stdio::piped())
         .stderr(Stdio::piped())
         .env("__CARGO_TEST_SETSID_PLEASE_DONT_USE_ELSEWHERE", "1");
    let mut child = cargo.spawn().unwrap();

    let mut sock = listener.accept().unwrap().0;
    ctrl_c(&mut child);

    assert!(!child.wait().unwrap().success());
    match sock.read(&mut [0; 10]) {
        Ok(n) => assert_eq!(n, 0),
        Err(e) => assert_eq!(e.kind(), io::ErrorKind::ConnectionReset),
    }

    // Ok so what we just did was spawn cargo that spawned a build script, then
    // we killed cargo in hopes of it killing the build script as well. If all
    // went well the build script is now dead. On Windows, however, this is
    // enforced with job objects which means that it may actually be in the
    // *process* of being torn down at this point.
    //
    // Now on Windows we can't completely remove a file until all handles to it
    // have been closed. Including those that represent running processes. So if
    // we were to return here then there may still be an open reference to some
    // file in the build directory. What we want to actually do is wait for the
    // build script to *complete* exit. Take care of that by blowing away the
    // build directory here, and panicking if we eventually spin too long
    // without being able to.
    for i in 0..10 {
        match fs::remove_dir_all(&p.root().join("target")) {
            Ok(()) => return,
            Err(e) => println!("attempt {}: {}", i, e),
        }
        thread::sleep(Duration::from_millis(100));
    }

    panic!("couldn't remove build directory after a few tries, seems like \
            we won't be able to!");
}

#[cfg(unix)]
fn ctrl_c(child: &mut Child) {
    use libc;

    let r = unsafe { libc::kill(-(child.id() as i32), libc::SIGINT) };
    if r < 0 {
        panic!("failed to kill: {}", io::Error::last_os_error());
    }
}

#[cfg(windows)]
fn ctrl_c(child: &mut Child) {
    child.kill().unwrap();
}
