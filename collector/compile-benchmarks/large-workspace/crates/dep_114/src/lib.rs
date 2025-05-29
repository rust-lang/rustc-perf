pub fn code() {
    println!("Hello from dep_114");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_114");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_114: {t}");
}

pub fn foo() {
    dep_48::code();
    dep_48::code_inlined();
    dep_48::code_generic(1u32);
    dep_14::code();
    dep_14::code_inlined();
    dep_14::code_generic(1u32);
    dep_54::code();
    dep_54::code_inlined();
    dep_54::code_generic(1u32);
}
