pub fn code() {
    println!("Hello from dep_29");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_29");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_29: {t}");
}

pub fn foo() {
    dep_8::code();
    dep_8::code_inlined();
    dep_8::code_generic(1u32);
}
