pub fn code() {
    println!("Hello from dep_781");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_781");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_781: {t}");
}

pub fn foo() {
    dep_215::code();
    dep_215::code_inlined();
    dep_215::code_generic(1u32);
    dep_344::code();
    dep_344::code_inlined();
    dep_344::code_generic(1u32);
    dep_250::code();
    dep_250::code_inlined();
    dep_250::code_generic(1u32);
    dep_258::code();
    dep_258::code_inlined();
    dep_258::code_generic(1u32);
    dep_329::code();
    dep_329::code_inlined();
    dep_329::code_generic(1u32);
}
