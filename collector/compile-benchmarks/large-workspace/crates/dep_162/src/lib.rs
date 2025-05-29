pub fn code() {
    println!("Hello from dep_162");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_162");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_162: {t}");
}

pub fn foo() {
    dep_81::code();
    dep_81::code_inlined();
    dep_81::code_generic(1u32);
    dep_140::code();
    dep_140::code_inlined();
    dep_140::code_generic(1u32);
    dep_139::code();
    dep_139::code_inlined();
    dep_139::code_generic(1u32);
    dep_79::code();
    dep_79::code_inlined();
    dep_79::code_generic(1u32);
}
