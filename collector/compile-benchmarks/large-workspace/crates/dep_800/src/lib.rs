pub fn code() {
    println!("Hello from dep_800");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_800");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_800: {t}");
}

pub fn foo() {
    dep_230::code();
    dep_230::code_inlined();
    dep_230::code_generic(1u32);
}
