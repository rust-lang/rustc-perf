pub fn code() {
    println!("Hello from dep_697");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_697");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_697: {t}");
}

pub fn foo() {
    dep_245::code();
    dep_245::code_inlined();
    dep_245::code_generic(1u32);
    dep_339::code();
    dep_339::code_inlined();
    dep_339::code_generic(1u32);
    dep_241::code();
    dep_241::code_inlined();
    dep_241::code_generic(1u32);
    dep_176::code();
    dep_176::code_inlined();
    dep_176::code_generic(1u32);
}
