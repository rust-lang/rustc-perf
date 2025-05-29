pub fn code() {
    println!("Hello from dep_828");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_828");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_828: {t}");
}

pub fn foo() {
    dep_229::code();
    dep_229::code_inlined();
    dep_229::code_generic(1u32);
    dep_241::code();
    dep_241::code_inlined();
    dep_241::code_generic(1u32);
}
