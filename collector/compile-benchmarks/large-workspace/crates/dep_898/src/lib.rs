pub fn code() {
    println!("Hello from dep_898");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_898");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_898: {t}");
}

pub fn foo() {
    dep_252::code();
    dep_252::code_inlined();
    dep_252::code_generic(1u32);
    dep_164::code();
    dep_164::code_inlined();
    dep_164::code_generic(1u32);
    dep_329::code();
    dep_329::code_inlined();
    dep_329::code_generic(1u32);
    dep_162::code();
    dep_162::code_inlined();
    dep_162::code_generic(1u32);
}
