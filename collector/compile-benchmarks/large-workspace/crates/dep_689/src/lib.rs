pub fn code() {
    println!("Hello from dep_689");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_689");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_689: {t}");
}

pub fn foo() {
    dep_189::code();
    dep_189::code_inlined();
    dep_189::code_generic(1u32);
}
