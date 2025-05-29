pub fn code() {
    println!("Hello from dep_816");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_816");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_816: {t}");
}

pub fn foo() {
    dep_283::code();
    dep_283::code_inlined();
    dep_283::code_generic(1u32);
    dep_309::code();
    dep_309::code_inlined();
    dep_309::code_generic(1u32);
    dep_247::code();
    dep_247::code_inlined();
    dep_247::code_generic(1u32);
    dep_386::code();
    dep_386::code_inlined();
    dep_386::code_generic(1u32);
}
