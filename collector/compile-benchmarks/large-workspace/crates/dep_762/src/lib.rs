pub fn code() {
    println!("Hello from dep_762");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_762");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_762: {t}");
}

pub fn foo() {
    dep_317::code();
    dep_317::code_inlined();
    dep_317::code_generic(1u32);
}
