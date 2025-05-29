pub fn code() {
    println!("Hello from dep_631");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_631");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_631: {t}");
}

pub fn foo() {
    dep_316::code();
    dep_316::code_inlined();
    dep_316::code_generic(1u32);
    dep_198::code();
    dep_198::code_inlined();
    dep_198::code_generic(1u32);
}
