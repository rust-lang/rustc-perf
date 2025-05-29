pub fn code() {
    println!("Hello from dep_783");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_783");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_783: {t}");
}

pub fn foo() {
    dep_379::code();
    dep_379::code_inlined();
    dep_379::code_generic(1u32);
    dep_275::code();
    dep_275::code_inlined();
    dep_275::code_generic(1u32);
}
