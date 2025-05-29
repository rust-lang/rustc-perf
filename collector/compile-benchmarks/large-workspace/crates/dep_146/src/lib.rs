pub fn code() {
    println!("Hello from dep_146");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_146");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_146: {t}");
}

pub fn foo() {
    dep_11::code();
    dep_11::code_inlined();
    dep_11::code_generic(1u32);
}
