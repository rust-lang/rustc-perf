pub fn code() {
    println!("Hello from dep_518");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_518");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_518: {t}");
}

pub fn foo() {
    dep_232::code();
    dep_232::code_inlined();
    dep_232::code_generic(1u32);
    dep_407::code();
    dep_407::code_inlined();
    dep_407::code_generic(1u32);
    dep_288::code();
    dep_288::code_inlined();
    dep_288::code_generic(1u32);
}
