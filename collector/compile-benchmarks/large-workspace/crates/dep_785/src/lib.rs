pub fn code() {
    println!("Hello from dep_785");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_785");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_785: {t}");
}

pub fn foo() {
    dep_407::code();
    dep_407::code_inlined();
    dep_407::code_generic(1u32);
    dep_282::code();
    dep_282::code_inlined();
    dep_282::code_generic(1u32);
}
