pub fn code() {
    println!("Hello from dep_590");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_590");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_590: {t}");
}

pub fn foo() {
    dep_408::code();
    dep_408::code_inlined();
    dep_408::code_generic(1u32);
    dep_216::code();
    dep_216::code_inlined();
    dep_216::code_generic(1u32);
    dep_233::code();
    dep_233::code_inlined();
    dep_233::code_generic(1u32);
}
