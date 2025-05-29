pub fn code() {
    println!("Hello from dep_137");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_137");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_137: {t}");
}

pub fn foo() {
    dep_27::code();
    dep_27::code_inlined();
    dep_27::code_generic(1u32);
    dep_51::code();
    dep_51::code_inlined();
    dep_51::code_generic(1u32);
    dep_47::code();
    dep_47::code_inlined();
    dep_47::code_generic(1u32);
    dep_52::code();
    dep_52::code_inlined();
    dep_52::code_generic(1u32);
    dep_54::code();
    dep_54::code_inlined();
    dep_54::code_generic(1u32);
}
