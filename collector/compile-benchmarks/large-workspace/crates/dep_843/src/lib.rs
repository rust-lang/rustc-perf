pub fn code() {
    println!("Hello from dep_843");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_843");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_843: {t}");
}

pub fn foo() {
    dep_247::code();
    dep_247::code_inlined();
    dep_247::code_generic(1u32);
    dep_319::code();
    dep_319::code_inlined();
    dep_319::code_generic(1u32);
    dep_202::code();
    dep_202::code_inlined();
    dep_202::code_generic(1u32);
}
