pub fn code() {
    println!("Hello from dep_741");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_741");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_741: {t}");
}

pub fn foo() {
    dep_320::code();
    dep_320::code_inlined();
    dep_320::code_generic(1u32);
    dep_212::code();
    dep_212::code_inlined();
    dep_212::code_generic(1u32);
}
