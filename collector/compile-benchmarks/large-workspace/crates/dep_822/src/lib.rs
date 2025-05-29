pub fn code() {
    println!("Hello from dep_822");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_822");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_822: {t}");
}

pub fn foo() {
    dep_203::code();
    dep_203::code_inlined();
    dep_203::code_generic(1u32);
    dep_300::code();
    dep_300::code_inlined();
    dep_300::code_generic(1u32);
    dep_332::code();
    dep_332::code_inlined();
    dep_332::code_generic(1u32);
    dep_235::code();
    dep_235::code_inlined();
    dep_235::code_generic(1u32);
}
