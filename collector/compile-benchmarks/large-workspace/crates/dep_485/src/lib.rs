pub fn code() {
    println!("Hello from dep_485");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_485");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_485: {t}");
}

pub fn foo() {
    dep_272::code();
    dep_272::code_inlined();
    dep_272::code_generic(1u32);
    dep_329::code();
    dep_329::code_inlined();
    dep_329::code_generic(1u32);
    dep_322::code();
    dep_322::code_inlined();
    dep_322::code_generic(1u32);
    dep_336::code();
    dep_336::code_inlined();
    dep_336::code_generic(1u32);
    dep_216::code();
    dep_216::code_inlined();
    dep_216::code_generic(1u32);
}
