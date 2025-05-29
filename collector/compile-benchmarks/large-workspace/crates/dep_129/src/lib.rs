pub fn code() {
    println!("Hello from dep_129");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_129");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_129: {t}");
}

pub fn foo() {
    dep_24::code();
    dep_24::code_inlined();
    dep_24::code_generic(1u32);
}
