pub fn code() {
    println!("Hello from dep_93");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_93");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_93: {t}");
}

pub fn foo() {
    dep_18::code();
    dep_18::code_inlined();
    dep_18::code_generic(1u32);
}
