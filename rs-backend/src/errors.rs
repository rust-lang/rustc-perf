error_chain! {
    links {
    }

    foreign_links {
        ::std::io::Error, Io, "io error";
        ::serde_json::Error, Json, "json error";
        ::chrono::ParseError, Chrono, "chrono parsing error";
    }

    errors {
    }
}
