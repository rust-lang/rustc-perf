pub fn code() {
    println!("Hello from dep_262");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_262");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_262: {t}");
}

pub fn foo() {
    dep_126::code();
    dep_126::code_inlined();
    dep_126::code_generic(1u32);
    dep_111::code();
    dep_111::code_inlined();
    dep_111::code_generic(1u32);
    dep_67::code();
    dep_67::code_inlined();
    dep_67::code_generic(1u32);
    dep_84::code();
    dep_84::code_inlined();
    dep_84::code_generic(1u32);
}
