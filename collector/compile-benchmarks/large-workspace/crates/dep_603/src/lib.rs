pub fn code() {
    println!("Hello from dep_603");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_603");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_603: {t}");
}

pub fn foo() {
    dep_230::code();
    dep_230::code_inlined();
    dep_230::code_generic(1u32);
    dep_219::code();
    dep_219::code_inlined();
    dep_219::code_generic(1u32);
    dep_213::code();
    dep_213::code_inlined();
    dep_213::code_generic(1u32);
    dep_180::code();
    dep_180::code_inlined();
    dep_180::code_generic(1u32);
}
