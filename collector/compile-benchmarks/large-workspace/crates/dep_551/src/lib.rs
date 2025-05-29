pub fn code() {
    println!("Hello from dep_551");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_551");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_551: {t}");
}

pub fn foo() {
    dep_273::code();
    dep_273::code_inlined();
    dep_273::code_generic(1u32);
    dep_243::code();
    dep_243::code_inlined();
    dep_243::code_generic(1u32);
    dep_332::code();
    dep_332::code_inlined();
    dep_332::code_generic(1u32);
    dep_354::code();
    dep_354::code_inlined();
    dep_354::code_generic(1u32);
}
