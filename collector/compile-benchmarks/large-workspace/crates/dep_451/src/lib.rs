pub fn code() {
    println!("Hello from dep_451");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_451");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_451: {t}");
}

pub fn foo() {
    dep_353::code();
    dep_353::code_inlined();
    dep_353::code_generic(1u32);
    dep_257::code();
    dep_257::code_inlined();
    dep_257::code_generic(1u32);
}
