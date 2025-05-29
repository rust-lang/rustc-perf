pub fn code() {
    println!("Hello from dep_712");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_712");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_712: {t}");
}

pub fn foo() {
    dep_284::code();
    dep_284::code_inlined();
    dep_284::code_generic(1u32);
    dep_194::code();
    dep_194::code_inlined();
    dep_194::code_generic(1u32);
    dep_272::code();
    dep_272::code_inlined();
    dep_272::code_generic(1u32);
    dep_228::code();
    dep_228::code_inlined();
    dep_228::code_generic(1u32);
    dep_171::code();
    dep_171::code_inlined();
    dep_171::code_generic(1u32);
}
