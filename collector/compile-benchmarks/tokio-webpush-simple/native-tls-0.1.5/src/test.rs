#[allow(unused_imports)]
use std::io::{Read, Write};
use std::net::{TcpStream, TcpListener};
use std::thread;
#[allow(unused_imports)]
use imp::TlsConnectorBuilderExt;

use super::*;

macro_rules! p {
    ($e:expr) => {
        match $e {
            Ok(r) => r,
            Err(e) => panic!("{:?}", e),
        }
    }
}

// This nested mod is needed for ios testing with rust-test-ios
mod tests {
    use super::*;

    #[test]
    fn connect_google() {
        let builder = p!(TlsConnector::builder());
        let builder = p!(builder.build());
        let s = p!(TcpStream::connect("google.com:443"));
        let mut socket = p!(builder.connect("google.com", s));

        p!(socket.write_all(b"GET / HTTP/1.0\r\n\r\n"));
        let mut result = vec![];
        p!(socket.read_to_end(&mut result));

        println!("{}", String::from_utf8_lossy(&result));
        assert!(result.starts_with(b"HTTP/1.0"));
        assert!(result.ends_with(b"</HTML>\r\n") || result.ends_with(b"</html>"));
    }

    #[test]
    fn connect_bad_hostname() {
        let builder = p!(TlsConnector::builder());
        let builder = p!(builder.build());
        let s = p!(TcpStream::connect("google.com:443"));
        builder.connect("goggle.com", s).unwrap_err();
    }

    #[test]
    fn connect_bad_hostname_ignored() {
        let builder = p!(TlsConnector::builder());
        let builder = p!(builder.build());
        let s = p!(TcpStream::connect("google.com:443"));
        builder.danger_connect_without_providing_domain_for_certificate_verification_and_server_name_indication(s).unwrap();
    }

    #[test]
    fn server() {
        let buf = include_bytes!("../test/identity.p12");
        let pkcs12 = p!(Pkcs12::from_der(buf, "mypass"));
        let builder = p!(TlsAcceptor::builder(pkcs12));
        let builder = p!(builder.build());

        let listener = p!(TcpListener::bind("0.0.0.0:0"));
        let port = p!(listener.local_addr()).port();

        let j = thread::spawn(move || {
            let socket = p!(listener.accept()).0;
            let mut socket = p!(builder.accept(socket));

            let mut buf = [0; 5];
            p!(socket.read_exact(&mut buf));
            assert_eq!(&buf, b"hello");

            p!(socket.write_all(b"world"));
        });

        let root_ca = include_bytes!("../test/root-ca.der");
        let root_ca = Certificate::from_der(root_ca).unwrap();

        let socket = p!(TcpStream::connect(("localhost", port)));
        let mut builder = p!(TlsConnector::builder());
        p!(builder.add_root_certificate(root_ca));
        let builder = p!(builder.build());
        let mut socket = p!(builder.connect("foobar.com", socket));

        p!(socket.write_all(b"hello"));
        let mut buf = vec![];
        p!(socket.read_to_end(&mut buf));
        assert_eq!(buf, b"world");

        p!(j.join());
    }

    #[test]
    #[cfg(not(target_os = "ios"))]
    fn server_pem() {
        let buf = include_bytes!("../test/identity.p12");
        let pkcs12 = p!(Pkcs12::from_der(buf, "mypass"));
        let builder = p!(TlsAcceptor::builder(pkcs12));
        let builder = p!(builder.build());

        let listener = p!(TcpListener::bind("0.0.0.0:0"));
        let port = p!(listener.local_addr()).port();

        let j = thread::spawn(move || {
            let socket = p!(listener.accept()).0;
            let mut socket = p!(builder.accept(socket));

            let mut buf = [0; 5];
            p!(socket.read_exact(&mut buf));
            assert_eq!(&buf, b"hello");

            p!(socket.write_all(b"world"));
        });

        let root_ca = include_bytes!("../test/root-ca.pem");
        let root_ca = Certificate::from_pem(root_ca).unwrap();

        let socket = p!(TcpStream::connect(("localhost", port)));
        let mut builder = p!(TlsConnector::builder());
        p!(builder.add_root_certificate(root_ca));
        let builder = p!(builder.build());
        let mut socket = p!(builder.connect("foobar.com", socket));

        p!(socket.write_all(b"hello"));
        let mut buf = vec![];
        p!(socket.read_to_end(&mut buf));
        assert_eq!(buf, b"world");

        p!(j.join());
    }

    #[test]
    fn server_tls11_only() {
        let buf = include_bytes!("../test/identity.p12");
        let pkcs12 = p!(Pkcs12::from_der(buf, "mypass"));
        let mut builder = p!(TlsAcceptor::builder(pkcs12));
        p!(builder.supported_protocols(&[Protocol::Tlsv11]));
        let builder = p!(builder.build());

        let listener = p!(TcpListener::bind("0.0.0.0:0"));
        let port = p!(listener.local_addr()).port();

        let j = thread::spawn(move || {
            let socket = p!(listener.accept()).0;
            let mut socket = p!(builder.accept(socket));

            let mut buf = [0; 5];
            p!(socket.read_exact(&mut buf));
            assert_eq!(&buf, b"hello");

            p!(socket.write_all(b"world"));
        });

        let root_ca = include_bytes!("../test/root-ca.der");
        let root_ca = Certificate::from_der(root_ca).unwrap();

        let socket = p!(TcpStream::connect(("localhost", port)));
        let mut builder = p!(TlsConnector::builder());
        p!(builder.add_root_certificate(root_ca));
        p!(builder.supported_protocols(&[Protocol::Tlsv11]));
        let builder = p!(builder.build());
        let mut socket = p!(builder.connect("foobar.com", socket));

        p!(socket.write_all(b"hello"));
        let mut buf = vec![];
        p!(socket.read_to_end(&mut buf));
        assert_eq!(buf, b"world");

        p!(j.join());
    }

    #[test]
    fn server_no_shared_protocol() {
        let buf = include_bytes!("../test/identity.p12");
        let pkcs12 = p!(Pkcs12::from_der(buf, "mypass"));
        let mut builder = p!(TlsAcceptor::builder(pkcs12));
        p!(builder.supported_protocols(&[Protocol::Tlsv12]));
        let builder = p!(builder.build());

        let listener = p!(TcpListener::bind("0.0.0.0:0"));
        let port = p!(listener.local_addr()).port();

        let j = thread::spawn(move || {
            let socket = p!(listener.accept()).0;
            assert!(builder.accept(socket).is_err());
        });

        let root_ca = include_bytes!("../test/root-ca.der");
        let root_ca = Certificate::from_der(root_ca).unwrap();

        let socket = p!(TcpStream::connect(("localhost", port)));
        let mut builder = p!(TlsConnector::builder());
        p!(builder.add_root_certificate(root_ca));
        p!(builder.supported_protocols(
            &[
                Protocol::Sslv3,
                Protocol::Tlsv10,
                Protocol::Tlsv11,
            ],
        ));
        let builder = p!(builder.build());
        assert!(builder.connect("foobar.com", socket).is_err());

        p!(j.join());
    }

    #[test]
    fn server_untrusted() {
        let buf = include_bytes!("../test/identity.p12");
        let pkcs12 = p!(Pkcs12::from_der(buf, "mypass"));
        let builder = p!(TlsAcceptor::builder(pkcs12));
        let builder = p!(builder.build());

        let listener = p!(TcpListener::bind("0.0.0.0:0"));
        let port = p!(listener.local_addr()).port();

        let j = thread::spawn(move || {
            let socket = p!(listener.accept()).0;
            // FIXME should assert error
            // https://github.com/steffengy/schannel-rs/issues/20
            let _ = builder.accept(socket);
        });

        let socket = p!(TcpStream::connect(("localhost", port)));
        let builder = p!(TlsConnector::builder());
        let builder = p!(builder.build());
        builder.connect("foobar.com", socket).unwrap_err();

        p!(j.join());
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn schannel_verify_callback() {
        let buf = include_bytes!("../test/identity.p12");
        let pkcs12 = p!(Pkcs12::from_der(buf, "mypass"));
        let builder = p!(TlsAcceptor::builder(pkcs12));
        let builder = p!(builder.build());

        let listener = p!(TcpListener::bind("0.0.0.0:0"));
        let port = p!(listener.local_addr()).port();

        let j = thread::spawn(move || {
            let socket = p!(listener.accept()).0;
            // FIXME should assert error
            // https://github.com/steffengy/schannel-rs/issues/20
            let _ = builder.accept(socket);
        });

        let socket = p!(TcpStream::connect(("localhost", port)));
        let mut builder = p!(TlsConnector::builder());
        builder.verify_callback(|validation_result| {
            assert!(validation_result.result().is_err());
            Ok(())
        });
        let builder = p!(builder.build());
        builder.connect("foobar.com", socket).unwrap();

        p!(j.join());
    }

    #[test]
    fn import_same_identity_multiple_times() {
        let buf = include_bytes!("../test/identity.p12");
        let _ = p!(Pkcs12::from_der(buf, "mypass"));
        let _ = p!(Pkcs12::from_der(buf, "mypass"));
    }

    #[test]
    fn shutdown() {
        let buf = include_bytes!("../test/identity.p12");
        let pkcs12 = p!(Pkcs12::from_der(buf, "mypass"));
        let builder = p!(TlsAcceptor::builder(pkcs12));
        let builder = p!(builder.build());

        let listener = p!(TcpListener::bind("0.0.0.0:0"));
        let port = p!(listener.local_addr()).port();

        let j = thread::spawn(move || {
            let socket = p!(listener.accept()).0;
            let mut socket = p!(builder.accept(socket));

            let mut buf = [0; 5];
            p!(socket.read_exact(&mut buf));
            assert_eq!(&buf, b"hello");

            assert_eq!(p!(socket.read(&mut buf)), 0);
            p!(socket.shutdown());
        });

        let root_ca = include_bytes!("../test/root-ca.der");
        let root_ca = Certificate::from_der(root_ca).unwrap();

        let socket = p!(TcpStream::connect(("localhost", port)));
        let mut builder = p!(TlsConnector::builder());
        p!(builder.add_root_certificate(root_ca));
        let builder = p!(builder.build());
        let mut socket = p!(builder.connect("foobar.com", socket));

        p!(socket.write_all(b"hello"));
        p!(socket.shutdown());

        p!(j.join());
    }
}
