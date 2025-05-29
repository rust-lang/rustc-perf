pub fn code() {
    println!("Hello from dep_67");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_67");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_67: {t}");
}

pub fn foo() {
    dep_54::code();
    dep_54::code_inlined();
    dep_54::code_generic(1u32);
    dep_44::code();
    dep_44::code_inlined();
    dep_44::code_generic(1u32);
    dep_34::code();
    dep_34::code_inlined();
    dep_34::code_generic(1u32);
    dep_30::code();
    dep_30::code_inlined();
    dep_30::code_generic(1u32);
}
