pub fn code() {
    println!("Hello from dep_109");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_109");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_109: {t}");
}

pub fn foo() {
    dep_54::code();
    dep_54::code_inlined();
    dep_54::code_generic(1u32);
    dep_51::code();
    dep_51::code_inlined();
    dep_51::code_generic(1u32);
    dep_11::code();
    dep_11::code_inlined();
    dep_11::code_generic(1u32);
    dep_48::code();
    dep_48::code_inlined();
    dep_48::code_generic(1u32);
    dep_21::code();
    dep_21::code_inlined();
    dep_21::code_generic(1u32);
}
