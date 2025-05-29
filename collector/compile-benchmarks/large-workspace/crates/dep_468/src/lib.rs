pub fn code() {
    println!("Hello from dep_468");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_468");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_468: {t}");
}

pub fn foo() {
    dep_340::code();
    dep_340::code_inlined();
    dep_340::code_generic(1u32);
    dep_242::code();
    dep_242::code_inlined();
    dep_242::code_generic(1u32);
    dep_190::code();
    dep_190::code_inlined();
    dep_190::code_generic(1u32);
    dep_400::code();
    dep_400::code_inlined();
    dep_400::code_generic(1u32);
}
