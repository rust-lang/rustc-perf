pub fn code() {
    println!("Hello from dep_613");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_613");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_613: {t}");
}

pub fn foo() {
    dep_337::code();
    dep_337::code_inlined();
    dep_337::code_generic(1u32);
}
