pub fn code() {
    println!("Hello from dep_16");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_16");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_16: {t}");
}

pub fn foo() {
    dep_0::code();
    dep_0::code_inlined();
    dep_0::code_generic(1u32);
    dep_7::code();
    dep_7::code_inlined();
    dep_7::code_generic(1u32);
}
