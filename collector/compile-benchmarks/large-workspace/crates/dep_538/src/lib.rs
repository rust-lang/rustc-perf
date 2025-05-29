pub fn code() {
    println!("Hello from dep_538");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_538");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_538: {t}");
}

pub fn foo() {
    dep_316::code();
    dep_316::code_inlined();
    dep_316::code_generic(1u32);
}
