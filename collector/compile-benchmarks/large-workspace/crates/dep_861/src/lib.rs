pub fn code() {
    println!("Hello from dep_861");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_861");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_861: {t}");
}

pub fn foo() {
    dep_230::code();
    dep_230::code_inlined();
    dep_230::code_generic(1u32);
    dep_193::code();
    dep_193::code_inlined();
    dep_193::code_generic(1u32);
    dep_327::code();
    dep_327::code_inlined();
    dep_327::code_generic(1u32);
    dep_339::code();
    dep_339::code_inlined();
    dep_339::code_generic(1u32);
    dep_232::code();
    dep_232::code_inlined();
    dep_232::code_generic(1u32);
}
