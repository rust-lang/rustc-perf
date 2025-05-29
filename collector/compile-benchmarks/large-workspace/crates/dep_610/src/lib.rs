pub fn code() {
    println!("Hello from dep_610");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_610");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_610: {t}");
}

pub fn foo() {
    dep_396::code();
    dep_396::code_inlined();
    dep_396::code_generic(1u32);
    dep_292::code();
    dep_292::code_inlined();
    dep_292::code_generic(1u32);
    dep_226::code();
    dep_226::code_inlined();
    dep_226::code_generic(1u32);
    dep_180::code();
    dep_180::code_inlined();
    dep_180::code_generic(1u32);
}
