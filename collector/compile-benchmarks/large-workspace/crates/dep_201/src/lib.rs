pub fn code() {
    println!("Hello from dep_201");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_201");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_201: {t}");
}

pub fn foo() {
    dep_65::code();
    dep_65::code_inlined();
    dep_65::code_generic(1u32);
}
