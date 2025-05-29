pub fn code() {
    println!("Hello from dep_814");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_814");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_814: {t}");
}

pub fn foo() {
    dep_371::code();
    dep_371::code_inlined();
    dep_371::code_generic(1u32);
    dep_226::code();
    dep_226::code_inlined();
    dep_226::code_generic(1u32);
    dep_238::code();
    dep_238::code_inlined();
    dep_238::code_generic(1u32);
    dep_197::code();
    dep_197::code_inlined();
    dep_197::code_generic(1u32);
}
