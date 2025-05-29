pub fn code() {
    println!("Hello from dep_703");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_703");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_703: {t}");
}

pub fn foo() {
    dep_317::code();
    dep_317::code_inlined();
    dep_317::code_generic(1u32);
    dep_243::code();
    dep_243::code_inlined();
    dep_243::code_generic(1u32);
    dep_305::code();
    dep_305::code_inlined();
    dep_305::code_generic(1u32);
    dep_284::code();
    dep_284::code_inlined();
    dep_284::code_generic(1u32);
}
