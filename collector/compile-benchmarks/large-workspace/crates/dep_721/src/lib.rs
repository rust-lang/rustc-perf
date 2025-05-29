pub fn code() {
    println!("Hello from dep_721");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_721");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_721: {t}");
}

pub fn foo() {
    dep_166::code();
    dep_166::code_inlined();
    dep_166::code_generic(1u32);
    dep_336::code();
    dep_336::code_inlined();
    dep_336::code_generic(1u32);
    dep_319::code();
    dep_319::code_inlined();
    dep_319::code_generic(1u32);
    dep_261::code();
    dep_261::code_inlined();
    dep_261::code_generic(1u32);
    dep_381::code();
    dep_381::code_inlined();
    dep_381::code_generic(1u32);
}
