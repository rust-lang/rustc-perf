pub fn code() {
    println!("Hello from dep_381");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_381");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_381: {t}");
}

pub fn foo() {
    dep_96::code();
    dep_96::code_inlined();
    dep_96::code_generic(1u32);
    dep_142::code();
    dep_142::code_inlined();
    dep_142::code_generic(1u32);
    dep_95::code();
    dep_95::code_inlined();
    dep_95::code_generic(1u32);
    dep_69::code();
    dep_69::code_inlined();
    dep_69::code_generic(1u32);
}
