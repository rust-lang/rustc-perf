pub fn code() {
    println!("Hello from dep_817");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_817");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_817: {t}");
}

pub fn foo() {
    dep_334::code();
    dep_334::code_inlined();
    dep_334::code_generic(1u32);
    dep_293::code();
    dep_293::code_inlined();
    dep_293::code_generic(1u32);
    dep_359::code();
    dep_359::code_inlined();
    dep_359::code_generic(1u32);
    dep_189::code();
    dep_189::code_inlined();
    dep_189::code_generic(1u32);
}
