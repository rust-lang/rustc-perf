pub fn code() {
    println!("Hello from dep_332");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_332");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_332: {t}");
}

pub fn foo() {
    dep_120::code();
    dep_120::code_inlined();
    dep_120::code_generic(1u32);
    dep_99::code();
    dep_99::code_inlined();
    dep_99::code_generic(1u32);
    dep_70::code();
    dep_70::code_inlined();
    dep_70::code_generic(1u32);
    dep_80::code();
    dep_80::code_inlined();
    dep_80::code_generic(1u32);
}
