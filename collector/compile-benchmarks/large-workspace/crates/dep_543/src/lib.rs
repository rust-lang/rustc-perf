pub fn code() {
    println!("Hello from dep_543");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_543");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_543: {t}");
}

pub fn foo() {
    dep_305::code();
    dep_305::code_inlined();
    dep_305::code_generic(1u32);
    dep_226::code();
    dep_226::code_inlined();
    dep_226::code_generic(1u32);
}
