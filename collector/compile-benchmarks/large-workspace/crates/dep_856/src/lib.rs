pub fn code() {
    println!("Hello from dep_856");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_856");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_856: {t}");
}

pub fn foo() {
    dep_322::code();
    dep_322::code_inlined();
    dep_322::code_generic(1u32);
    dep_241::code();
    dep_241::code_inlined();
    dep_241::code_generic(1u32);
}
