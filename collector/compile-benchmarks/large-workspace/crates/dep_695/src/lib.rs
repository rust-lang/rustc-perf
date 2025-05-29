pub fn code() {
    println!("Hello from dep_695");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_695");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_695: {t}");
}

pub fn foo() {
    dep_268::code();
    dep_268::code_inlined();
    dep_268::code_generic(1u32);
    dep_364::code();
    dep_364::code_inlined();
    dep_364::code_generic(1u32);
}
