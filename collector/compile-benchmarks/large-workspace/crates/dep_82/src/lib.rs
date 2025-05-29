pub fn code() {
    println!("Hello from dep_82");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_82");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_82: {t}");
}

pub fn foo() {
    dep_31::code();
    dep_31::code_inlined();
    dep_31::code_generic(1u32);
    dep_41::code();
    dep_41::code_inlined();
    dep_41::code_generic(1u32);
    dep_19::code();
    dep_19::code_inlined();
    dep_19::code_generic(1u32);
}
