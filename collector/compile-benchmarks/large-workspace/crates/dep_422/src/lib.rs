pub fn code() {
    println!("Hello from dep_422");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_422");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_422: {t}");
}

pub fn foo() {
    dep_310::code();
    dep_310::code_inlined();
    dep_310::code_generic(1u32);
    dep_241::code();
    dep_241::code_inlined();
    dep_241::code_generic(1u32);
    dep_325::code();
    dep_325::code_inlined();
    dep_325::code_generic(1u32);
}
