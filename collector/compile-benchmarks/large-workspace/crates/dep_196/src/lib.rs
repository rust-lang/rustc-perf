pub fn code() {
    println!("Hello from dep_196");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_196");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_196: {t}");
}

pub fn foo() {
    dep_67::code();
    dep_67::code_inlined();
    dep_67::code_generic(1u32);
    dep_63::code();
    dep_63::code_inlined();
    dep_63::code_generic(1u32);
    dep_132::code();
    dep_132::code_inlined();
    dep_132::code_generic(1u32);
    dep_137::code();
    dep_137::code_inlined();
    dep_137::code_generic(1u32);
}
