pub fn code() {
    println!("Hello from dep_198");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_198");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_198: {t}");
}

pub fn foo() {
    dep_136::code();
    dep_136::code_inlined();
    dep_136::code_generic(1u32);
    dep_66::code();
    dep_66::code_inlined();
    dep_66::code_generic(1u32);
    dep_114::code();
    dep_114::code_inlined();
    dep_114::code_generic(1u32);
    dep_117::code();
    dep_117::code_inlined();
    dep_117::code_generic(1u32);
    dep_143::code();
    dep_143::code_inlined();
    dep_143::code_generic(1u32);
}
