pub fn code() {
    println!("Hello from dep_239");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_239");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_239: {t}");
}

pub fn foo() {
    dep_78::code();
    dep_78::code_inlined();
    dep_78::code_generic(1u32);
    dep_63::code();
    dep_63::code_inlined();
    dep_63::code_generic(1u32);
    dep_102::code();
    dep_102::code_inlined();
    dep_102::code_generic(1u32);
    dep_80::code();
    dep_80::code_inlined();
    dep_80::code_generic(1u32);
    dep_122::code();
    dep_122::code_inlined();
    dep_122::code_generic(1u32);
}
