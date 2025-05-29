pub fn code() {
    println!("Hello from dep_823");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_823");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_823: {t}");
}

pub fn foo() {
    dep_184::code();
    dep_184::code_inlined();
    dep_184::code_generic(1u32);
    dep_270::code();
    dep_270::code_inlined();
    dep_270::code_generic(1u32);
    dep_332::code();
    dep_332::code_inlined();
    dep_332::code_generic(1u32);
    dep_396::code();
    dep_396::code_inlined();
    dep_396::code_generic(1u32);
    dep_390::code();
    dep_390::code_inlined();
    dep_390::code_generic(1u32);
}
