pub fn code() {
    println!("Hello from dep_42");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_42");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_42: {t}");
}

pub fn foo() {
    dep_7::code();
    dep_7::code_inlined();
    dep_7::code_generic(1u32);
    dep_5::code();
    dep_5::code_inlined();
    dep_5::code_generic(1u32);
}
