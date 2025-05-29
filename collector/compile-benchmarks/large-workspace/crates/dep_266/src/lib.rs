pub fn code() {
    println!("Hello from dep_266");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_266");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_266: {t}");
}

pub fn foo() {
    dep_81::code();
    dep_81::code_inlined();
    dep_81::code_generic(1u32);
    dep_125::code();
    dep_125::code_inlined();
    dep_125::code_generic(1u32);
    dep_108::code();
    dep_108::code_inlined();
    dep_108::code_generic(1u32);
}
