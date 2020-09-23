// A regression test for #75992.
// Nested asnyc blocks produce an exponentially sized type tree with a lot of duplicates.
//
// Created by @kellerkindt in https://github.com/rust-lang/rust/issues/75992#issuecomment-682595159.

pub async fn h0(v: &String, x: &u64) { println!("{} {}", v, x) }
pub async fn h1(v: &String, x: &u64) { h0(v, x).await }
pub async fn h2(v: &String, x: &u64) { h1(v, x).await }
pub async fn h3(v: &String, x: &u64) { h2(v, x).await }
pub async fn h4(v: &String, x: &u64) { h3(v, x).await }
pub async fn h5(v: &String, x: &u64) { h4(v, x).await }
pub async fn h6(v: &String, x: &u64) { h5(v, x).await }
pub async fn h7(v: &String, x: &u64) { h6(v, x).await }
pub async fn h8(v: &String, x: &u64) { h7(v, x).await }
pub async fn h9(v: &String, x: &u64) { h8(v, x).await }

pub async fn h10(v: &String, x: &u64) { h9(v, x).await }
pub async fn h11(v: &String, x: &u64) { h10(v, x).await }
pub async fn h12(v: &String, x: &u64) { h11(v, x).await }
pub async fn h13(v: &String, x: &u64) { h12(v, x).await }
pub async fn h14(v: &String, x: &u64) { h13(v, x).await }
pub async fn h15(v: &String, x: &u64) { h14(v, x).await }
pub async fn h16(v: &String, x: &u64) { h15(v, x).await }
pub async fn h17(v: &String, x: &u64) { h16(v, x).await }
pub async fn h18(v: &String, x: &u64) { h17(v, x).await }
pub async fn h19(v: &String, x: &u64) { h18(v, x).await }


macro_rules! async_recursive {
    (13, $inner:expr) => { async { async_recursive!(12, $inner) }.await };
    (12, $inner:expr) => { async { async_recursive!(11, $inner) }.await };
    (11, $inner:expr) => { async { async_recursive!(10, $inner) }.await };
    (10, $inner:expr) => { async { async_recursive!(9, $inner) }.await };

    (9, $inner:expr) => { async { async_recursive!(8, $inner) }.await };
    (8, $inner:expr) => { async { async_recursive!(7, $inner) }.await };
    (7, $inner:expr) => { async { async_recursive!(6, $inner) }.await };
    (6, $inner:expr) => { async { async_recursive!(5, $inner) }.await };
    (5, $inner:expr) => { async { async_recursive!(4, $inner) }.await };
    (4, $inner:expr) => { async { async_recursive!(3, $inner) }.await };
    (3, $inner:expr) => { async { async_recursive!(2, $inner) }.await };
    (2, $inner:expr) => { async { async_recursive!(1, $inner) }.await };
    (1, $inner:expr) => { async { async_recursive!(0, $inner) }.await };
    (0, $inner:expr) => { async { h19(&String::from("owo"), &0).await; $inner }.await };
}

async fn f() {
    async_recursive!(13, println!("hello"));
}

pub fn foo() {
    let _ = f();
}
