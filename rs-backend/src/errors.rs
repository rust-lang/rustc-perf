error_chain! {
    links {
    }

    foreign_links {
        ::std::io::Error, Io;
        ::serde_json::Error, Json;
        ::chrono::ParseError, Chrono;
    }

    errors {
    }
}
