pub fn code() {
    println!("Hello from dep_180");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_180");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_180: {t}");
}

pub fn foo() {
    dep_63::code();
    dep_63::code_inlined();
    dep_63::code_generic(1u32);
    dep_156::code();
    dep_156::code_inlined();
    dep_156::code_generic(1u32);
    dep_76::code();
    dep_76::code_inlined();
    dep_76::code_generic(1u32);
    dep_110::code();
    dep_110::code_inlined();
    dep_110::code_generic(1u32);
    dep_129::code();
    dep_129::code_inlined();
    dep_129::code_generic(1u32);
}
