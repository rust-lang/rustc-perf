pub fn code() {
    println!("Hello from dep_212");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_212");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_212: {t}");
}

pub fn foo() {
    dep_137::code();
    dep_137::code_inlined();
    dep_137::code_generic(1u32);
    dep_99::code();
    dep_99::code_inlined();
    dep_99::code_generic(1u32);
    dep_71::code();
    dep_71::code_inlined();
    dep_71::code_generic(1u32);
    dep_92::code();
    dep_92::code_inlined();
    dep_92::code_generic(1u32);
    dep_105::code();
    dep_105::code_inlined();
    dep_105::code_generic(1u32);
}
