pub fn code() {
    println!("Hello from dep_863");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_863");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_863: {t}");
}

pub fn foo() {
    dep_279::code();
    dep_279::code_inlined();
    dep_279::code_generic(1u32);
    dep_297::code();
    dep_297::code_inlined();
    dep_297::code_generic(1u32);
    dep_293::code();
    dep_293::code_inlined();
    dep_293::code_generic(1u32);
}
