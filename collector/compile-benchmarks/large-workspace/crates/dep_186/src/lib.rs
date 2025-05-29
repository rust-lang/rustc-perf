pub fn code() {
    println!("Hello from dep_186");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_186");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_186: {t}");
}

pub fn foo() {
    dep_79::code();
    dep_79::code_inlined();
    dep_79::code_generic(1u32);
    dep_159::code();
    dep_159::code_inlined();
    dep_159::code_generic(1u32);
    dep_128::code();
    dep_128::code_inlined();
    dep_128::code_generic(1u32);
    dep_117::code();
    dep_117::code_inlined();
    dep_117::code_generic(1u32);
    dep_135::code();
    dep_135::code_inlined();
    dep_135::code_generic(1u32);
}
