pub fn code() {
    println!("Hello from dep_696");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_696");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_696: {t}");
}

pub fn foo() {
    dep_211::code();
    dep_211::code_inlined();
    dep_211::code_generic(1u32);
    dep_398::code();
    dep_398::code_inlined();
    dep_398::code_generic(1u32);
    dep_189::code();
    dep_189::code_inlined();
    dep_189::code_generic(1u32);
    dep_408::code();
    dep_408::code_inlined();
    dep_408::code_generic(1u32);
}
