use futures::{stream, TryStreamExt};

fn main() {
    stream::empty::<Result<(), ()>>()
        .inspect_ok(|_| ())
        .inspect_ok(|_| ())
        .inspect_ok(|_| ())
        .inspect_ok(|_| ())
        .inspect_ok(|_| ())
        .inspect_ok(|_| ())
        .inspect_ok(|_| ())
        .inspect_ok(|_| ())
        .inspect_ok(|_| ())
        .inspect_ok(|_| ())
        .inspect_ok(|_| ())
        .inspect_ok(|_| ())
        .inspect_ok(|_| ())
        .inspect_ok(|_| ())
        .inspect_ok(|_| ())
        .inspect_ok(|_| ())
        .inspect_ok(|_| ())
        .inspect_ok(|_| ());
}
