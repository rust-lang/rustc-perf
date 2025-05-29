pub fn code() {
    println!("Hello from dep_735");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_735");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_735: {t}");
}

pub fn foo() {
    dep_231::code();
    dep_231::code_inlined();
    dep_231::code_generic(1u32);
    dep_407::code();
    dep_407::code_inlined();
    dep_407::code_generic(1u32);
    dep_328::code();
    dep_328::code_inlined();
    dep_328::code_generic(1u32);
    dep_273::code();
    dep_273::code_inlined();
    dep_273::code_generic(1u32);
    dep_315::code();
    dep_315::code_inlined();
    dep_315::code_generic(1u32);
}
