pub fn code() {
    println!("Hello from dep_295");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_295");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_295: {t}");
}

pub fn foo() {
    dep_63::code();
    dep_63::code_inlined();
    dep_63::code_generic(1u32);
    dep_104::code();
    dep_104::code_inlined();
    dep_104::code_generic(1u32);
    dep_159::code();
    dep_159::code_inlined();
    dep_159::code_generic(1u32);
}
