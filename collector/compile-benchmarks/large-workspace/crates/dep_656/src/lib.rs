pub fn code() {
    println!("Hello from dep_656");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_656");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_656: {t}");
}

pub fn foo() {
    dep_208::code();
    dep_208::code_inlined();
    dep_208::code_generic(1u32);
}
