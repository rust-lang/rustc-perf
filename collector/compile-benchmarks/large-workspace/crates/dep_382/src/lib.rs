pub fn code() {
    println!("Hello from dep_382");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_382");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_382: {t}");
}

pub fn foo() {
    dep_76::code();
    dep_76::code_inlined();
    dep_76::code_generic(1u32);
    dep_83::code();
    dep_83::code_inlined();
    dep_83::code_generic(1u32);
    dep_150::code();
    dep_150::code_inlined();
    dep_150::code_generic(1u32);
    dep_156::code();
    dep_156::code_inlined();
    dep_156::code_generic(1u32);
    dep_105::code();
    dep_105::code_inlined();
    dep_105::code_generic(1u32);
}
