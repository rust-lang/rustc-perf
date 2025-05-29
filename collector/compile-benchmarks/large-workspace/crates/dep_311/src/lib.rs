pub fn code() {
    println!("Hello from dep_311");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_311");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_311: {t}");
}

pub fn foo() {
    dep_111::code();
    dep_111::code_inlined();
    dep_111::code_generic(1u32);
    dep_81::code();
    dep_81::code_inlined();
    dep_81::code_generic(1u32);
    dep_103::code();
    dep_103::code_inlined();
    dep_103::code_generic(1u32);
}
