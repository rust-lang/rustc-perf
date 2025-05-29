pub fn code() {
    println!("Hello from dep_708");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_708");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_708: {t}");
}

pub fn foo() {
    dep_320::code();
    dep_320::code_inlined();
    dep_320::code_generic(1u32);
    dep_211::code();
    dep_211::code_inlined();
    dep_211::code_generic(1u32);
    dep_164::code();
    dep_164::code_inlined();
    dep_164::code_generic(1u32);
}
