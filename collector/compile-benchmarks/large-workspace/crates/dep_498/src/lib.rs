pub fn code() {
    println!("Hello from dep_498");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_498");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_498: {t}");
}

pub fn foo() {
    dep_271::code();
    dep_271::code_inlined();
    dep_271::code_generic(1u32);
    dep_288::code();
    dep_288::code_inlined();
    dep_288::code_generic(1u32);
    dep_353::code();
    dep_353::code_inlined();
    dep_353::code_generic(1u32);
    dep_236::code();
    dep_236::code_inlined();
    dep_236::code_generic(1u32);
}
