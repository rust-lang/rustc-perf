pub fn code() {
    println!("Hello from dep_638");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_638");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_638: {t}");
}

pub fn foo() {
    dep_290::code();
    dep_290::code_inlined();
    dep_290::code_generic(1u32);
    dep_173::code();
    dep_173::code_inlined();
    dep_173::code_generic(1u32);
    dep_257::code();
    dep_257::code_inlined();
    dep_257::code_generic(1u32);
}
