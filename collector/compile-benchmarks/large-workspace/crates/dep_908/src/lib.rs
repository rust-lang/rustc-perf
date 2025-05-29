pub fn code() {
    println!("Hello from dep_908");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_908");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_908: {t}");
}

pub fn foo() {
    dep_212::code();
    dep_212::code_inlined();
    dep_212::code_generic(1u32);
}
