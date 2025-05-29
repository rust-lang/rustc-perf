pub fn code() {
    println!("Hello from dep_45");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_45");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_45: {t}");
}

pub fn foo() {
    dep_4::code();
    dep_4::code_inlined();
    dep_4::code_generic(1u32);
}
