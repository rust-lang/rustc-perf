pub fn code() {
    println!("Hello from dep_584");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_584");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_584: {t}");
}

pub fn foo() {
    dep_273::code();
    dep_273::code_inlined();
    dep_273::code_generic(1u32);
}
