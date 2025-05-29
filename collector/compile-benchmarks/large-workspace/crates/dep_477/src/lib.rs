pub fn code() {
    println!("Hello from dep_477");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_477");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_477: {t}");
}

pub fn foo() {
    dep_313::code();
    dep_313::code_inlined();
    dep_313::code_generic(1u32);
    dep_308::code();
    dep_308::code_inlined();
    dep_308::code_generic(1u32);
    dep_181::code();
    dep_181::code_inlined();
    dep_181::code_generic(1u32);
    dep_408::code();
    dep_408::code_inlined();
    dep_408::code_generic(1u32);
    dep_280::code();
    dep_280::code_inlined();
    dep_280::code_generic(1u32);
}
