pub fn code() {
    println!("Hello from dep_473");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_473");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_473: {t}");
}

pub fn foo() {
    dep_251::code();
    dep_251::code_inlined();
    dep_251::code_generic(1u32);
    dep_301::code();
    dep_301::code_inlined();
    dep_301::code_generic(1u32);
}
