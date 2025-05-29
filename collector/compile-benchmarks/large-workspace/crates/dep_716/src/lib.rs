pub fn code() {
    println!("Hello from dep_716");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_716");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_716: {t}");
}

pub fn foo() {
    dep_268::code();
    dep_268::code_inlined();
    dep_268::code_generic(1u32);
    dep_407::code();
    dep_407::code_inlined();
    dep_407::code_generic(1u32);
    dep_332::code();
    dep_332::code_inlined();
    dep_332::code_generic(1u32);
}
