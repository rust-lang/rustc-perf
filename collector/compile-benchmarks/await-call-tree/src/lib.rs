#![type_length_limit="15000000"]
#![allow(unused)]

macro_rules! mk_async_fn {
    ($f:ident $g:ident) => {
        async fn $g() -> i32 {
            $f().await;
            $f().await;
            $f().await
        }
    }
}

async fn a() -> i32 { 1 }

mk_async_fn!(a b);
mk_async_fn!(b c);
mk_async_fn!(c d);
mk_async_fn!(d e);
mk_async_fn!(e f);
mk_async_fn!(f g);
mk_async_fn!(g h);
