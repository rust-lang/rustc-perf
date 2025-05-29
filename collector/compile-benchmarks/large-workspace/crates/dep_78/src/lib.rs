pub fn code() {
    println!("Hello from dep_78");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_78");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_78: {t}");
}

pub fn foo() {
    dep_50::code();
    dep_50::code_inlined();
    dep_50::code_generic(1u32);
}
