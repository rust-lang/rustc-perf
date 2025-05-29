pub fn code() {
    println!("Hello from dep_820");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_820");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_820: {t}");
}

pub fn foo() {
    dep_352::code();
    dep_352::code_inlined();
    dep_352::code_generic(1u32);
    dep_281::code();
    dep_281::code_inlined();
    dep_281::code_generic(1u32);
    dep_228::code();
    dep_228::code_inlined();
    dep_228::code_generic(1u32);
    dep_381::code();
    dep_381::code_inlined();
    dep_381::code_generic(1u32);
    dep_193::code();
    dep_193::code_inlined();
    dep_193::code_generic(1u32);
}
