pub fn code() {
    println!("Hello from dep_71");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_71");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_71: {t}");
}

pub fn foo() {
    dep_18::code();
    dep_18::code_inlined();
    dep_18::code_generic(1u32);
    dep_44::code();
    dep_44::code_inlined();
    dep_44::code_generic(1u32);
    dep_24::code();
    dep_24::code_inlined();
    dep_24::code_generic(1u32);
    dep_54::code();
    dep_54::code_inlined();
    dep_54::code_generic(1u32);
}
