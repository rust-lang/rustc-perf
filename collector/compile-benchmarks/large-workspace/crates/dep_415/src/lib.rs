pub fn code() {
    println!("Hello from dep_415");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_415");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_415: {t}");
}

pub fn foo() {
    dep_208::code();
    dep_208::code_inlined();
    dep_208::code_generic(1u32);
    dep_307::code();
    dep_307::code_inlined();
    dep_307::code_generic(1u32);
    dep_258::code();
    dep_258::code_inlined();
    dep_258::code_generic(1u32);
}
