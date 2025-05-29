pub fn code() {
    println!("Hello from dep_342");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_342");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_342: {t}");
}

pub fn foo() {
    dep_124::code();
    dep_124::code_inlined();
    dep_124::code_generic(1u32);
    dep_64::code();
    dep_64::code_inlined();
    dep_64::code_generic(1u32);
    dep_117::code();
    dep_117::code_inlined();
    dep_117::code_generic(1u32);
    dep_90::code();
    dep_90::code_inlined();
    dep_90::code_generic(1u32);
    dep_142::code();
    dep_142::code_inlined();
    dep_142::code_generic(1u32);
}
