// A test case from rust-lang/rust#72408.
// Nesting closures produce exponentially sized type tree with a lot of duplicates.

#![type_length_limit="469762040"]

fn dup(f: impl Fn(i32) -> i32) -> impl Fn(i32) -> i32 {
    move |a| f(a * 2)
}

pub fn foo() {
    let f = |a| a;

    let f = dup(f);
    let f = dup(f);
    let f = dup(f);
    let f = dup(f);
    let f = dup(f);

    let f = dup(f);
    let f = dup(f);
    let f = dup(f);
    let f = dup(f);
    let f = dup(f);

    let f = dup(f);
    let f = dup(f);
    let f = dup(f);
    let f = dup(f);
    let f = dup(f);

    let f = dup(f);
    let f = dup(f);
    let f = dup(f);
    let f = dup(f);
    let f = dup(f);

    let f = dup(f);
    let f = dup(f);
    let f = dup(f);
    let f = dup(f);
    let f = dup(f);

    println!("Type size was at least {}", f(1));
}
