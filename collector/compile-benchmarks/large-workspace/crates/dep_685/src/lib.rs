pub fn code() {
    println!("Hello from dep_685");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_685");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_685: {t}");
}

pub fn foo() {
    dep_333::code();
    dep_333::code_inlined();
    dep_333::code_generic(1u32);
    dep_184::code();
    dep_184::code_inlined();
    dep_184::code_generic(1u32);
    dep_225::code();
    dep_225::code_inlined();
    dep_225::code_generic(1u32);
    dep_291::code();
    dep_291::code_inlined();
    dep_291::code_generic(1u32);
}
