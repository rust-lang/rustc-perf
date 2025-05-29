pub fn code() {
    println!("Hello from dep_223");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_223");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_223: {t}");
}

pub fn foo() {
    dep_68::code();
    dep_68::code_inlined();
    dep_68::code_generic(1u32);
    dep_82::code();
    dep_82::code_inlined();
    dep_82::code_generic(1u32);
    dep_111::code();
    dep_111::code_inlined();
    dep_111::code_generic(1u32);
    dep_119::code();
    dep_119::code_inlined();
    dep_119::code_generic(1u32);
    dep_112::code();
    dep_112::code_inlined();
    dep_112::code_generic(1u32);
}
