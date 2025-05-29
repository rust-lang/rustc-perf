pub fn code() {
    println!("Hello from dep_650");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_650");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_650: {t}");
}

pub fn foo() {
    dep_386::code();
    dep_386::code_inlined();
    dep_386::code_generic(1u32);
    dep_169::code();
    dep_169::code_inlined();
    dep_169::code_generic(1u32);
    dep_237::code();
    dep_237::code_inlined();
    dep_237::code_generic(1u32);
}
