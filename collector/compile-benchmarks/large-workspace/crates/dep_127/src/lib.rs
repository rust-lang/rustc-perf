pub fn code() {
    println!("Hello from dep_127");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_127");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_127: {t}");
}

pub fn foo() {
    dep_42::code();
    dep_42::code_inlined();
    dep_42::code_generic(1u32);
    dep_36::code();
    dep_36::code_inlined();
    dep_36::code_generic(1u32);
    dep_23::code();
    dep_23::code_inlined();
    dep_23::code_generic(1u32);
    dep_58::code();
    dep_58::code_inlined();
    dep_58::code_generic(1u32);
}
