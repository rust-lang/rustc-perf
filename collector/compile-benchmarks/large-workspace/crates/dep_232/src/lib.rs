pub fn code() {
    println!("Hello from dep_232");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_232");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_232: {t}");
}

pub fn foo() {
    dep_157::code();
    dep_157::code_inlined();
    dep_157::code_generic(1u32);
    dep_156::code();
    dep_156::code_inlined();
    dep_156::code_generic(1u32);
    dep_88::code();
    dep_88::code_inlined();
    dep_88::code_generic(1u32);
    dep_68::code();
    dep_68::code_inlined();
    dep_68::code_generic(1u32);
    dep_135::code();
    dep_135::code_inlined();
    dep_135::code_generic(1u32);
}
