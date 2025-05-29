pub fn code() {
    println!("Hello from dep_699");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_699");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_699: {t}");
}

pub fn foo() {
    dep_192::code();
    dep_192::code_inlined();
    dep_192::code_generic(1u32);
    dep_327::code();
    dep_327::code_inlined();
    dep_327::code_generic(1u32);
    dep_377::code();
    dep_377::code_inlined();
    dep_377::code_generic(1u32);
    dep_207::code();
    dep_207::code_inlined();
    dep_207::code_generic(1u32);
    dep_385::code();
    dep_385::code_inlined();
    dep_385::code_generic(1u32);
}
