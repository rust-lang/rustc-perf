pub fn code() {
    println!("Hello from dep_218");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_218");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_218: {t}");
}

pub fn foo() {
    dep_89::code();
    dep_89::code_inlined();
    dep_89::code_generic(1u32);
    dep_128::code();
    dep_128::code_inlined();
    dep_128::code_generic(1u32);
    dep_145::code();
    dep_145::code_inlined();
    dep_145::code_generic(1u32);
    dep_83::code();
    dep_83::code_inlined();
    dep_83::code_generic(1u32);
}
