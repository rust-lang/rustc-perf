pub fn code() {
    println!("Hello from dep_524");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_524");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_524: {t}");
}

pub fn foo() {
    dep_172::code();
    dep_172::code_inlined();
    dep_172::code_generic(1u32);
}
