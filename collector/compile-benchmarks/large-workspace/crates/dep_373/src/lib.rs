pub fn code() {
    println!("Hello from dep_373");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_373");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_373: {t}");
}

pub fn foo() {
    dep_81::code();
    dep_81::code_inlined();
    dep_81::code_generic(1u32);
}
