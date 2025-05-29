pub fn code() {
    println!("Hello from dep_751");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_751");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_751: {t}");
}

pub fn foo() {
    dep_160::code();
    dep_160::code_inlined();
    dep_160::code_generic(1u32);
    dep_208::code();
    dep_208::code_inlined();
    dep_208::code_generic(1u32);
}
