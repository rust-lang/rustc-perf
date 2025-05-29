pub fn code() {
    println!("Hello from dep_344");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_344");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_344: {t}");
}

pub fn foo() {
    dep_154::code();
    dep_154::code_inlined();
    dep_154::code_generic(1u32);
    dep_70::code();
    dep_70::code_inlined();
    dep_70::code_generic(1u32);
    dep_118::code();
    dep_118::code_inlined();
    dep_118::code_generic(1u32);
    dep_137::code();
    dep_137::code_inlined();
    dep_137::code_generic(1u32);
    dep_159::code();
    dep_159::code_inlined();
    dep_159::code_generic(1u32);
}
