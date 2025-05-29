pub fn code() {
    println!("Hello from dep_788");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_788");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_788: {t}");
}

pub fn foo() {
    dep_295::code();
    dep_295::code_inlined();
    dep_295::code_generic(1u32);
    dep_308::code();
    dep_308::code_inlined();
    dep_308::code_generic(1u32);
    dep_251::code();
    dep_251::code_inlined();
    dep_251::code_generic(1u32);
}
