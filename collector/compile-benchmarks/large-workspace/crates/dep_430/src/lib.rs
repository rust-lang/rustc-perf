pub fn code() {
    println!("Hello from dep_430");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_430");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_430: {t}");
}

pub fn foo() {
    dep_351::code();
    dep_351::code_inlined();
    dep_351::code_generic(1u32);
    dep_326::code();
    dep_326::code_inlined();
    dep_326::code_generic(1u32);
    dep_257::code();
    dep_257::code_inlined();
    dep_257::code_generic(1u32);
    dep_208::code();
    dep_208::code_inlined();
    dep_208::code_generic(1u32);
}
