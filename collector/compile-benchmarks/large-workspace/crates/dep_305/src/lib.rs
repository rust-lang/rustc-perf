pub fn code() {
    println!("Hello from dep_305");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_305");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_305: {t}");
}

pub fn foo() {
    dep_76::code();
    dep_76::code_inlined();
    dep_76::code_generic(1u32);
    dep_119::code();
    dep_119::code_inlined();
    dep_119::code_generic(1u32);
    dep_135::code();
    dep_135::code_inlined();
    dep_135::code_generic(1u32);
    dep_140::code();
    dep_140::code_inlined();
    dep_140::code_generic(1u32);
    dep_156::code();
    dep_156::code_inlined();
    dep_156::code_generic(1u32);
}
