pub fn code() {
    println!("Hello from dep_461");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_461");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_461: {t}");
}

pub fn foo() {
    dep_268::code();
    dep_268::code_inlined();
    dep_268::code_generic(1u32);
    dep_197::code();
    dep_197::code_inlined();
    dep_197::code_generic(1u32);
    dep_391::code();
    dep_391::code_inlined();
    dep_391::code_generic(1u32);
}
