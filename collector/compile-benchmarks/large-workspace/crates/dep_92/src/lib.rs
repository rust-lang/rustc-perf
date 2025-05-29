pub fn code() {
    println!("Hello from dep_92");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_92");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_92: {t}");
}

pub fn foo() {
    dep_54::code();
    dep_54::code_inlined();
    dep_54::code_generic(1u32);
}
