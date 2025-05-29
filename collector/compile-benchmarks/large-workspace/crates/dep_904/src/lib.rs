pub fn code() {
    println!("Hello from dep_904");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_904");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_904: {t}");
}

pub fn foo() {
    dep_400::code();
    dep_400::code_inlined();
    dep_400::code_generic(1u32);
    dep_191::code();
    dep_191::code_inlined();
    dep_191::code_generic(1u32);
    dep_383::code();
    dep_383::code_inlined();
    dep_383::code_generic(1u32);
    dep_382::code();
    dep_382::code_inlined();
    dep_382::code_generic(1u32);
    dep_317::code();
    dep_317::code_inlined();
    dep_317::code_generic(1u32);
}
