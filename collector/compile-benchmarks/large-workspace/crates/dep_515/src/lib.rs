pub fn code() {
    println!("Hello from dep_515");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_515");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_515: {t}");
}

pub fn foo() {
    dep_258::code();
    dep_258::code_inlined();
    dep_258::code_generic(1u32);
    dep_182::code();
    dep_182::code_inlined();
    dep_182::code_generic(1u32);
    dep_205::code();
    dep_205::code_inlined();
    dep_205::code_generic(1u32);
}
