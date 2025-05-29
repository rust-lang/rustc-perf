pub fn code() {
    println!("Hello from dep_864");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_864");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_864: {t}");
}

pub fn foo() {
    dep_305::code();
    dep_305::code_inlined();
    dep_305::code_generic(1u32);
    dep_304::code();
    dep_304::code_inlined();
    dep_304::code_generic(1u32);
    dep_235::code();
    dep_235::code_inlined();
    dep_235::code_generic(1u32);
    dep_160::code();
    dep_160::code_inlined();
    dep_160::code_generic(1u32);
}
