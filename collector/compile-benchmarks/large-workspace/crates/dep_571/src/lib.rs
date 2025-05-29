pub fn code() {
    println!("Hello from dep_571");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_571");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_571: {t}");
}

pub fn foo() {
    dep_298::code();
    dep_298::code_inlined();
    dep_298::code_generic(1u32);
    dep_169::code();
    dep_169::code_inlined();
    dep_169::code_generic(1u32);
    dep_248::code();
    dep_248::code_inlined();
    dep_248::code_generic(1u32);
}
