pub fn code() {
    println!("Hello from dep_634");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_634");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_634: {t}");
}

pub fn foo() {
    dep_339::code();
    dep_339::code_inlined();
    dep_339::code_generic(1u32);
}
