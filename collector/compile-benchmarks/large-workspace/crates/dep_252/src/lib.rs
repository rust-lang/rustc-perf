pub fn code() {
    println!("Hello from dep_252");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_252");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_252: {t}");
}

pub fn foo() {
    dep_131::code();
    dep_131::code_inlined();
    dep_131::code_generic(1u32);
    dep_102::code();
    dep_102::code_inlined();
    dep_102::code_generic(1u32);
    dep_149::code();
    dep_149::code_inlined();
    dep_149::code_generic(1u32);
    dep_154::code();
    dep_154::code_inlined();
    dep_154::code_generic(1u32);
    dep_66::code();
    dep_66::code_inlined();
    dep_66::code_generic(1u32);
}
