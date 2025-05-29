pub fn code() {
    println!("Hello from dep_448");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_448");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_448: {t}");
}

pub fn foo() {
    dep_174::code();
    dep_174::code_inlined();
    dep_174::code_generic(1u32);
    dep_189::code();
    dep_189::code_inlined();
    dep_189::code_generic(1u32);
    dep_241::code();
    dep_241::code_inlined();
    dep_241::code_generic(1u32);
}
