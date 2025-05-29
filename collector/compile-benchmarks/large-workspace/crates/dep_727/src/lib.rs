pub fn code() {
    println!("Hello from dep_727");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_727");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_727: {t}");
}

pub fn foo() {
    dep_295::code();
    dep_295::code_inlined();
    dep_295::code_generic(1u32);
    dep_277::code();
    dep_277::code_inlined();
    dep_277::code_generic(1u32);
    dep_184::code();
    dep_184::code_inlined();
    dep_184::code_generic(1u32);
}
