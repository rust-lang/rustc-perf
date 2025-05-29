pub fn code() {
    println!("Hello from dep_501");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_501");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_501: {t}");
}

pub fn foo() {
    dep_164::code();
    dep_164::code_inlined();
    dep_164::code_generic(1u32);
    dep_368::code();
    dep_368::code_inlined();
    dep_368::code_generic(1u32);
}
