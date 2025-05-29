pub fn code() {
    println!("Hello from dep_837");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_837");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_837: {t}");
}

pub fn foo() {
    dep_365::code();
    dep_365::code_inlined();
    dep_365::code_generic(1u32);
    dep_203::code();
    dep_203::code_inlined();
    dep_203::code_generic(1u32);
}
