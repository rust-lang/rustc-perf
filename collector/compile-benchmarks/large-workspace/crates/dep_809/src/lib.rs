pub fn code() {
    println!("Hello from dep_809");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_809");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_809: {t}");
}

pub fn foo() {
    dep_196::code();
    dep_196::code_inlined();
    dep_196::code_generic(1u32);
}
