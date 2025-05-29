pub fn code() {
    println!("Hello from dep_559");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_559");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_559: {t}");
}

pub fn foo() {
    dep_231::code();
    dep_231::code_inlined();
    dep_231::code_generic(1u32);
    dep_407::code();
    dep_407::code_inlined();
    dep_407::code_generic(1u32);
}
