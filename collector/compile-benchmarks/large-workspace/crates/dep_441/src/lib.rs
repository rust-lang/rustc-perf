pub fn code() {
    println!("Hello from dep_441");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_441");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_441: {t}");
}

pub fn foo() {
    dep_322::code();
    dep_322::code_inlined();
    dep_322::code_generic(1u32);
    dep_169::code();
    dep_169::code_inlined();
    dep_169::code_generic(1u32);
    dep_198::code();
    dep_198::code_inlined();
    dep_198::code_generic(1u32);
}
