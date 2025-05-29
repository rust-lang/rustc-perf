pub fn code() {
    println!("Hello from dep_877");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_877");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_877: {t}");
}

pub fn foo() {
    dep_381::code();
    dep_381::code_inlined();
    dep_381::code_generic(1u32);
    dep_338::code();
    dep_338::code_inlined();
    dep_338::code_generic(1u32);
    dep_185::code();
    dep_185::code_inlined();
    dep_185::code_generic(1u32);
    dep_269::code();
    dep_269::code_inlined();
    dep_269::code_generic(1u32);
}
