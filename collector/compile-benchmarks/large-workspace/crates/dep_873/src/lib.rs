pub fn code() {
    println!("Hello from dep_873");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_873");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_873: {t}");
}

pub fn foo() {
    dep_382::code();
    dep_382::code_inlined();
    dep_382::code_generic(1u32);
    dep_243::code();
    dep_243::code_inlined();
    dep_243::code_generic(1u32);
    dep_240::code();
    dep_240::code_inlined();
    dep_240::code_generic(1u32);
    dep_354::code();
    dep_354::code_inlined();
    dep_354::code_generic(1u32);
    dep_396::code();
    dep_396::code_inlined();
    dep_396::code_generic(1u32);
}
