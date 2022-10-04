//! An abstraction over platform-specific TLS implementations.
//!
//! Many applications require TLS/SSL communication in one form or another as
//! part of their implementation, but finding a library for this isn't always
//! trivial! The purpose of this crate is to provide a seamless integration
//! experience on all platforms with a cross-platform API that deals with all
//! the underlying details for you.
//!
//! ```toml
//! # Cargo.toml
//! [dependencies]
//! native-tls = "0.1"
//! ```
//!
//! # How is this implemented?
//!
//! This crate uses SChannel on Windows (via the `schannel` crate), Secure
//! Transport on OSX (via the `security-framework` crate), and OpenSSL (via the
//! `openssl` crate) on all other platforms. Future futures may also enable
//! other TLS frameworks as well, but these initial libraries are likely to
//! remain as the defaults.
//!
//! If you know you're on a particular platform then you can use the
//! platform-specific extension traits in this crate to configure the underlying
//! details of that platform. For example OpenSSL may have more options for
//! configuration than Secure Transport. By default, though, the API of this
//! crate works across all platforms.
//!
//! Note that this crate also strives to be secure-by-default. For example when
//! using OpenSSL it will configure validation callbacks to ensure that
//! hostnames match certificates, use strong ciphers, etc. This implies that
//! this crate is *not* just a thin abstraction around the underlying libraries,
//! but also an implementation that strives to strike reasonable defaults.
//!
//! # Supported features
//!
//! This crate supports the following features out of the box:
//!
//! * TLS/SSL client communication
//! * TLS/SSL server communication
//! * PKCS#12 encoded server identities
//! * Secure-by-default for client and server
//!     * Includes hostname verification for clients
//! * Supports asynchronous I/O for both the server and the client
//!
//! Each implementation may support more features which can be accessed through
//! the extension traits in the `backend` module.
//!
//! # Examples
//!
//! To connect as a client to a remote server:
//!
//! ```rust
//! use native_tls::TlsConnector;
//! use std::io::{Read, Write};
//! use std::net::TcpStream;
//!
//! let connector = TlsConnector::builder().unwrap().build().unwrap();
//!
//! let stream = TcpStream::connect("google.com:443").unwrap();
//! let mut stream = connector.connect("google.com", stream).unwrap();
//!
//! stream.write_all(b"GET / HTTP/1.0\r\n\r\n").unwrap();
//! let mut res = vec![];
//! stream.read_to_end(&mut res).unwrap();
//! println!("{}", String::from_utf8_lossy(&res));
//! ```
//!
//! To accept connections as a server from remote clients:
//!
//! ```rust,no_run
//! use native_tls::{Pkcs12, TlsAcceptor, TlsStream};
//! use std::fs::File;
//! use std::io::{Read};
//! use std::net::{TcpListener, TcpStream};
//! use std::sync::Arc;
//! use std::thread;
//!
//! let mut file = File::open("identity.pfx").unwrap();
//! let mut pkcs12 = vec![];
//! file.read_to_end(&mut pkcs12).unwrap();
//! let pkcs12 = Pkcs12::from_der(&pkcs12, "hunter2").unwrap();
//!
//! let listener = TcpListener::bind("0.0.0.0:8443").unwrap();
//! let acceptor = TlsAcceptor::builder(pkcs12).unwrap().build().unwrap();
//! let acceptor = Arc::new(acceptor);
//!
//! fn handle_client(stream: TlsStream<TcpStream>) {
//!     // ...
//! }
//!
//! for stream in listener.incoming() {
//!     match stream {
//!         Ok(stream) => {
//!             let acceptor = acceptor.clone();
//!             thread::spawn(move || {
//!                 let stream = acceptor.accept(stream).unwrap();
//!                 handle_client(stream);
//!             });
//!         }
//!         Err(e) => { /* connection failed */ }
//!     }
//! }
//! ```
#![doc(html_root_url="https://docs.rs/native-tls/0.1.5")]
#![warn(missing_docs)]

#[macro_use]
#[cfg(any(target_os = "macos", target_os = "ios"))]
extern crate lazy_static;

use std::any::Any;
use std::error;
use std::error::Error as StdError;
use std::io;
use std::fmt;
use std::result;
use std::marker::PhantomData;

pub mod backend;

// #[cfg(any(target_os = "macos", target_os = "ios"))]
// #[path = "imp/security_framework.rs"]
// mod imp;
// #[cfg(target_os = "windows")]
// #[path = "imp/schannel.rs"]
// mod imp;
// #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
// #[path = "imp/openssl.rs"]
// mod imp;

#[cfg(test)]
mod test;

/// A typedef of the result-type returned by many methods.
pub type Result<T> = result::Result<T, Error>;

/// An error returned from the TLS implementation.
pub struct Error(());

impl error::Error for Error {
    fn description(&self) -> &str {
        ""
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

/// A PKCS #12 archive.
pub struct Pkcs12(());

impl Pkcs12 {
    /// Parses a DER-formatted PKCS #12 archive, using the specified password to decrypt the key.
    ///
    /// The archive should contain a leaf certificate and its private key, as well any intermediate
    /// certificates that should be sent to clients to allow them to build a chain to a trusted
    /// root. The chain certificates should be in order from the leaf certificate towards the root.
    ///
    /// PKCS #12 archives typically have the file extension `.p12` or `.pfx`, and can be created
    /// with the OpenSSL `pkcs12` tool:
    ///
    /// ```bash
    /// openssl pkcs12 -export -out identity.pfx -inkey key.pem -in cert.pem -certfile chain_certs.pem
    /// ```
    pub fn from_der(_der: &[u8], _password: &str) -> Result<Pkcs12> {
        unimplemented!()
    }
}

/// An X509 certificate.
pub struct Certificate(());

impl Certificate {
    /// Parses a DER-formatted X509 certificate.
    pub fn from_der(der: &[u8]) -> Result<Certificate> {
        loop {}
    }
    /// Parses a PEM-formatted X509 certificate.
    /// If the PEM file contains more than one certificate the last one is used
    /// and the others are ignored.
    pub fn from_pem(der: &[u8]) -> Result<Certificate> {
        loop {}
    }
}

/// A TLS stream which has been interrupted midway through the handshake process.
pub struct MidHandshakeTlsStream<S>(PhantomData<S>);

impl<S> fmt::Debug for MidHandshakeTlsStream<S>
where
    S: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl<S> MidHandshakeTlsStream<S>
where
    S: io::Read + io::Write,
{
    /// Returns a shared reference to the inner stream.
    pub fn get_ref(&self) -> &S {
        loop {}
    }

    /// Returns a mutable reference to the inner stream.
    pub fn get_mut(&mut self) -> &mut S {
        loop {}
    }

    /// Restarts the handshake process.
    ///
    /// If the handshake completes successfully then the negotiated stream is
    /// returned. If there is a problem, however, then an error is returned.
    /// Note that the error may not be fatal. For example if the underlying
    /// stream is an asynchronous one then `HandshakeError::Interrupted` may
    /// just mean to wait for more I/O to happen later.
    pub fn handshake(self) -> result::Result<TlsStream<S>, HandshakeError<S>> {
        loop {}
    }
}

/// An error returned from `ClientBuilder::handshake`.
#[derive(Debug)]
pub enum HandshakeError<S> {
    /// A fatal error.
    Failure(Error),

    /// A stream interrupted midway through the handshake process due to a
    /// `WouldBlock` error.
    ///
    /// Note that this is not a fatal error and it should be safe to call
    /// `handshake` at a later time once the stream is ready to perform I/O
    /// again.
    Interrupted(MidHandshakeTlsStream<S>),
}

impl<S> error::Error for HandshakeError<S>
where
    S: Any + fmt::Debug,
{
    fn description(&self) -> &str {
        match *self {
            HandshakeError::Failure(ref e) => e.description(),
            HandshakeError::Interrupted(_) => "the handshake process was interrupted",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            HandshakeError::Failure(ref e) => Some(e),
            HandshakeError::Interrupted(_) => None,
        }
    }
}

impl<S> fmt::Display for HandshakeError<S>
where
    S: Any + fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(fmt.write_str(self.description()));
        if let Some(cause) = self.cause() {
            try!(write!(fmt, ": {}", cause));
        }
        Ok(())
    }
}

/// SSL/TLS protocol versions.
#[derive(Debug, Copy, Clone)]
pub enum Protocol {
    /// The SSL 3.0 protocol.
    ///
    /// # Warning
    ///
    /// SSL 3.0 has severe security flaws, and should not be used unless absolutely necessary. If
    /// you are not sure if you need to enable this protocol, you should not.
    Sslv3,
    /// The TLS 1.0 protocol.
    Tlsv10,
    /// The TLS 1.1 protocol.
    Tlsv11,
    /// The TLS 1.2 protocol.
    Tlsv12,
    #[doc(hidden)]
    __NonExhaustive,
}

/// A builder for `TlsConnector`s.
pub struct TlsConnectorBuilder(());

impl TlsConnectorBuilder {
    /// Sets the identity to be used for client certificate authentication.
    pub fn identity(&mut self, _pkcs12: Pkcs12) -> Result<&mut TlsConnectorBuilder> {
        loop {}
    }

    /// Sets the protocols which the connector will support.
    ///
    /// The protocols supported by default are currently TLS 1.0, TLS 1.1, and TLS 1.2, though this
    /// is subject to change.
    pub fn supported_protocols(
        &mut self,
        _protocols: &[Protocol],
    ) -> Result<&mut TlsConnectorBuilder> {
        loop {}
    }

    /// Adds a certificate to the set of roots that the connector will trust.
    ///
    /// The connector will use the system's trust root by default. This method can be used to add
    /// to that set when communicating with servers not trusted by the system.
    pub fn add_root_certificate(&mut self, cert: Certificate) -> Result<&mut TlsConnectorBuilder> {
        loop {}
    }

    /// Consumes the builder, returning a `TlsConnector`.
    pub fn build(self) -> Result<TlsConnector> {
        loop {}
    }
}

/// A builder for client-side TLS connections.
///
/// # Examples
///
/// ```rust
/// use native_tls::TlsConnector;
/// use std::io::{Read, Write};
/// use std::net::TcpStream;
///
/// let connector = TlsConnector::builder().unwrap().build().unwrap();
///
/// let stream = TcpStream::connect("google.com:443").unwrap();
/// let mut stream = connector.connect("google.com", stream).unwrap();
///
/// stream.write_all(b"GET / HTTP/1.0\r\n\r\n").unwrap();
/// let mut res = vec![];
/// stream.read_to_end(&mut res).unwrap();
/// println!("{}", String::from_utf8_lossy(&res));
/// ```
#[derive(Clone)]
pub struct TlsConnector(());

impl TlsConnector {
    /// Returns a new builder for a `TlsConnector`.
    pub fn builder() -> Result<TlsConnectorBuilder> {
        loop {}
    }

    /// Initiates a TLS handshake.
    ///
    /// The provided domain will be used for both SNI and certificate hostname
    /// validation.
    ///
    /// If the socket is nonblocking and a `WouldBlock` error is returned during
    /// the handshake, a `HandshakeError::Interrupted` error will be returned
    /// which can be used to restart the handshake when the socket is ready
    /// again.
    pub fn connect<S>(
        &self,
        domain: &str,
        stream: S,
    ) -> result::Result<TlsStream<S>, HandshakeError<S>>
    where
        S: io::Read + io::Write,
    {
        loop {}
    }

    /// Like `connect`, but does not validate the server's domain name against its certificate.
    ///
    /// # Warning
    ///
    /// You should think very carefully before you use this method. If hostname verification is not
    /// used, *any* valid certificate for *any* site will be trusted for use from any other. This
    /// introduces a significant vulnerability to man-in-the-middle attacks.
    pub fn danger_connect_without_providing_domain_for_certificate_verification_and_server_name_indication<S>(
            &self, stream: S) -> result::Result<TlsStream<S>, HandshakeError<S>>
        where S: io::Read + io::Write
    {
        loop {}
    }
}

/// A builder for `TlsAcceptor`s.
pub struct TlsAcceptorBuilder(());

impl TlsAcceptorBuilder {
    /// Sets the protocols which the acceptor will support.
    ///
    /// The protocols supported by default are currently TLS 1.0, TLS 1.1, and TLS 1.2, though this
    /// is subject to change.
    pub fn supported_protocols(
        &mut self,
        protocols: &[Protocol],
    ) -> Result<&mut TlsAcceptorBuilder> {
        loop {}
    }

    /// Consumes the builder, returning a `TlsAcceptor`.
    pub fn build(self) -> Result<TlsAcceptor> {
        loop {}
    }
}

/// A builder for server-side TLS connections.
///
/// # Examples
///
/// ```rust,no_run
/// use native_tls::{Pkcs12, TlsAcceptor, TlsStream};
/// use std::fs::File;
/// use std::io::{Read};
/// use std::net::{TcpListener, TcpStream};
/// use std::sync::Arc;
/// use std::thread;
///
/// let mut file = File::open("identity.pfx").unwrap();
/// let mut pkcs12 = vec![];
/// file.read_to_end(&mut pkcs12).unwrap();
/// let pkcs12 = Pkcs12::from_der(&pkcs12, "hunter2").unwrap();
///
/// let listener = TcpListener::bind("0.0.0.0:8443").unwrap();
/// let acceptor = TlsAcceptor::builder(pkcs12).unwrap().build().unwrap();
/// let acceptor = Arc::new(acceptor);
///
/// fn handle_client(stream: TlsStream<TcpStream>) {
///     // ...
/// }
///
/// for stream in listener.incoming() {
///     match stream {
///         Ok(stream) => {
///             let acceptor = acceptor.clone();
///             thread::spawn(move || {
///                 let stream = acceptor.accept(stream).unwrap();
///                 handle_client(stream);
///             });
///         }
///         Err(e) => { /* connection failed */ }
///     }
/// }
/// ```
#[derive(Clone)]
pub struct TlsAcceptor(());

impl TlsAcceptor {
    /// Returns a new builder for a `TlsAcceptor`.
    ///
    /// This builder is created with a key/certificate pair in the `pkcs12`
    /// archived passed in. The returned builder will use that key/certificate
    /// to send to clients which it connects to.
    pub fn builder(pkcs12: Pkcs12) -> Result<TlsAcceptorBuilder> {
        loop {}
    }

    /// Initiates a TLS handshake.
    ///
    /// If the socket is nonblocking and a `WouldBlock` error is returned during
    /// the handshake, a `HandshakeError::Interrupted` error will be returned
    /// which can be used to restart the handshake when the socket is ready
    /// again.
    pub fn accept<S>(&self, stream: S) -> result::Result<TlsStream<S>, HandshakeError<S>>
    where
        S: io::Read + io::Write,
    {
        loop {}
    }
}

/// A stream managing a TLS session.
pub struct TlsStream<S>(PhantomData<S>);

impl<S: fmt::Debug> fmt::Debug for TlsStream<S> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        loop {}
    }
}

impl<S: io::Read + io::Write> TlsStream<S> {
    /// Returns a shared reference to the inner stream.
    pub fn get_ref(&self) -> &S {
        loop {}
    }

    /// Returns a mutable reference to the inner stream.
    pub fn get_mut(&mut self) -> &mut S {
        loop {}
    }

    /// Returns the number of bytes that can be read without resulting in any
    /// network calls.
    pub fn buffered_read_size(&self) -> Result<usize> {
        loop {}
    }

    /// Shuts down the TLS session.
    pub fn shutdown(&mut self) -> io::Result<()> {
        loop {}
    }
}

impl<S: io::Read + io::Write> io::Read for TlsStream<S> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        loop {}
    }
}

impl<S: io::Read + io::Write> io::Write for TlsStream<S> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        loop {}
    }

    fn flush(&mut self) -> io::Result<()> {
        loop {}
    }
}

fn _check_kinds() {
    loop {}
}
