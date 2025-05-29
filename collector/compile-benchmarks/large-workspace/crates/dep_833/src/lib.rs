pub fn code() {
    println!("Hello from dep_833");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_833");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_833: {t}");
}

pub fn foo() {
    dep_225::code();
    dep_225::code_inlined();
    dep_225::code_generic(1u32);
    dep_310::code();
    dep_310::code_inlined();
    dep_310::code_generic(1u32);
    dep_182::code();
    dep_182::code_inlined();
    dep_182::code_generic(1u32);
    dep_233::code();
    dep_233::code_inlined();
    dep_233::code_generic(1u32);
}
