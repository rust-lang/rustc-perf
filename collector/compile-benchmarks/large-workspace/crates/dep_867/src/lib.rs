pub fn code() {
    println!("Hello from dep_867");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_867");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_867: {t}");
}

pub fn foo() {
    dep_170::code();
    dep_170::code_inlined();
    dep_170::code_generic(1u32);
}
