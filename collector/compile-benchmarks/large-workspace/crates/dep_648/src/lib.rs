pub fn code() {
    println!("Hello from dep_648");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_648");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_648: {t}");
}

pub fn foo() {
    dep_391::code();
    dep_391::code_inlined();
    dep_391::code_generic(1u32);
    dep_192::code();
    dep_192::code_inlined();
    dep_192::code_generic(1u32);
    dep_171::code();
    dep_171::code_inlined();
    dep_171::code_generic(1u32);
    dep_265::code();
    dep_265::code_inlined();
    dep_265::code_generic(1u32);
}
