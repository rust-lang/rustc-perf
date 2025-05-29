pub fn code() {
    println!("Hello from dep_798");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_798");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_798: {t}");
}

pub fn foo() {
    dep_355::code();
    dep_355::code_inlined();
    dep_355::code_generic(1u32);
    dep_347::code();
    dep_347::code_inlined();
    dep_347::code_generic(1u32);
    dep_242::code();
    dep_242::code_inlined();
    dep_242::code_generic(1u32);
}
