pub fn code() {
    println!("Hello from dep_807");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_807");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_807: {t}");
}

pub fn foo() {
    dep_161::code();
    dep_161::code_inlined();
    dep_161::code_generic(1u32);
}
