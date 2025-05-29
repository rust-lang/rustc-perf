pub fn code() {
    println!("Hello from dep_379");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_379");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_379: {t}");
}

pub fn foo() {
    dep_67::code();
    dep_67::code_inlined();
    dep_67::code_generic(1u32);
    dep_69::code();
    dep_69::code_inlined();
    dep_69::code_generic(1u32);
    dep_143::code();
    dep_143::code_inlined();
    dep_143::code_generic(1u32);
    dep_135::code();
    dep_135::code_inlined();
    dep_135::code_generic(1u32);
}
