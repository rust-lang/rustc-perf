pub fn code() {
    println!("Hello from dep_69");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_69");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_69: {t}");
}

pub fn foo() {
    dep_31::code();
    dep_31::code_inlined();
    dep_31::code_generic(1u32);
    dep_23::code();
    dep_23::code_inlined();
    dep_23::code_generic(1u32);
    dep_49::code();
    dep_49::code_inlined();
    dep_49::code_generic(1u32);
    dep_52::code();
    dep_52::code_inlined();
    dep_52::code_generic(1u32);
}
