pub fn code() {
    println!("Hello from dep_506");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_506");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_506: {t}");
}

pub fn foo() {
    dep_333::code();
    dep_333::code_inlined();
    dep_333::code_generic(1u32);
    dep_325::code();
    dep_325::code_inlined();
    dep_325::code_generic(1u32);
    dep_272::code();
    dep_272::code_inlined();
    dep_272::code_generic(1u32);
    dep_164::code();
    dep_164::code_inlined();
    dep_164::code_generic(1u32);
    dep_203::code();
    dep_203::code_inlined();
    dep_203::code_generic(1u32);
}
