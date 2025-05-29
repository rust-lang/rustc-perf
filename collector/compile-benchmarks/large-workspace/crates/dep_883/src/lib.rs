pub fn code() {
    println!("Hello from dep_883");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_883");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_883: {t}");
}

pub fn foo() {
    dep_278::code();
    dep_278::code_inlined();
    dep_278::code_generic(1u32);
    dep_352::code();
    dep_352::code_inlined();
    dep_352::code_generic(1u32);
    dep_322::code();
    dep_322::code_inlined();
    dep_322::code_generic(1u32);
}
