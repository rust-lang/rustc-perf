pub fn code() {
    println!("Hello from dep_767");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_767");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_767: {t}");
}

pub fn foo() {
    dep_328::code();
    dep_328::code_inlined();
    dep_328::code_generic(1u32);
    dep_273::code();
    dep_273::code_inlined();
    dep_273::code_generic(1u32);
    dep_239::code();
    dep_239::code_inlined();
    dep_239::code_generic(1u32);
    dep_175::code();
    dep_175::code_inlined();
    dep_175::code_generic(1u32);
    dep_283::code();
    dep_283::code_inlined();
    dep_283::code_generic(1u32);
}
