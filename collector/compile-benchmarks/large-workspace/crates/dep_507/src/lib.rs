pub fn code() {
    println!("Hello from dep_507");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_507");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_507: {t}");
}

pub fn foo() {
    dep_332::code();
    dep_332::code_inlined();
    dep_332::code_generic(1u32);
    dep_248::code();
    dep_248::code_inlined();
    dep_248::code_generic(1u32);
    dep_164::code();
    dep_164::code_inlined();
    dep_164::code_generic(1u32);
    dep_265::code();
    dep_265::code_inlined();
    dep_265::code_generic(1u32);
}
