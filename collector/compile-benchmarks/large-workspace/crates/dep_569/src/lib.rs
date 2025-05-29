pub fn code() {
    println!("Hello from dep_569");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_569");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_569: {t}");
}

pub fn foo() {
    dep_372::code();
    dep_372::code_inlined();
    dep_372::code_generic(1u32);
    dep_247::code();
    dep_247::code_inlined();
    dep_247::code_generic(1u32);
}
