pub fn code() {
    println!("Hello from dep_879");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_879");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_879: {t}");
}

pub fn foo() {
    dep_172::code();
    dep_172::code_inlined();
    dep_172::code_generic(1u32);
    dep_216::code();
    dep_216::code_inlined();
    dep_216::code_generic(1u32);
    dep_215::code();
    dep_215::code_inlined();
    dep_215::code_generic(1u32);
    dep_285::code();
    dep_285::code_inlined();
    dep_285::code_generic(1u32);
    dep_188::code();
    dep_188::code_inlined();
    dep_188::code_generic(1u32);
}
