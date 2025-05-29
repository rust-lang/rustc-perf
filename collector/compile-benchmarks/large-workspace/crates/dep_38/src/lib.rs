pub fn code() {
    println!("Hello from dep_38");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_38");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_38: {t}");
}

pub fn foo() {
    dep_5::code();
    dep_5::code_inlined();
    dep_5::code_generic(1u32);
    dep_3::code();
    dep_3::code_inlined();
    dep_3::code_generic(1u32);
}
