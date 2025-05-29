pub fn code() {
    println!("Hello from dep_319");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_319");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_319: {t}");
}

pub fn foo() {
    dep_156::code();
    dep_156::code_inlined();
    dep_156::code_generic(1u32);
    dep_152::code();
    dep_152::code_inlined();
    dep_152::code_generic(1u32);
    dep_109::code();
    dep_109::code_inlined();
    dep_109::code_generic(1u32);
    dep_71::code();
    dep_71::code_inlined();
    dep_71::code_generic(1u32);
    dep_103::code();
    dep_103::code_inlined();
    dep_103::code_generic(1u32);
}
