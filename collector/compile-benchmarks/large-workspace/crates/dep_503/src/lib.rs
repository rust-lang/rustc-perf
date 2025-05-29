pub fn code() {
    println!("Hello from dep_503");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_503");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_503: {t}");
}

pub fn foo() {
    dep_293::code();
    dep_293::code_inlined();
    dep_293::code_generic(1u32);
    dep_313::code();
    dep_313::code_inlined();
    dep_313::code_generic(1u32);
}
