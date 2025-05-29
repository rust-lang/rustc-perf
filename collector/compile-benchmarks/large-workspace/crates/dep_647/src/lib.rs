pub fn code() {
    println!("Hello from dep_647");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_647");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_647: {t}");
}

pub fn foo() {
    dep_225::code();
    dep_225::code_inlined();
    dep_225::code_generic(1u32);
    dep_392::code();
    dep_392::code_inlined();
    dep_392::code_generic(1u32);
    dep_314::code();
    dep_314::code_inlined();
    dep_314::code_generic(1u32);
}
