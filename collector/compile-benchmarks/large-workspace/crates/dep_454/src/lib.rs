pub fn code() {
    println!("Hello from dep_454");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_454");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_454: {t}");
}

pub fn foo() {
    dep_178::code();
    dep_178::code_inlined();
    dep_178::code_generic(1u32);
    dep_355::code();
    dep_355::code_inlined();
    dep_355::code_generic(1u32);
    dep_254::code();
    dep_254::code_inlined();
    dep_254::code_generic(1u32);
    dep_287::code();
    dep_287::code_inlined();
    dep_287::code_generic(1u32);
}
