pub fn code() {
    println!("Hello from dep_850");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_850");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_850: {t}");
}

pub fn foo() {
    dep_324::code();
    dep_324::code_inlined();
    dep_324::code_generic(1u32);
    dep_300::code();
    dep_300::code_inlined();
    dep_300::code_generic(1u32);
    dep_388::code();
    dep_388::code_inlined();
    dep_388::code_generic(1u32);
    dep_366::code();
    dep_366::code_inlined();
    dep_366::code_generic(1u32);
    dep_336::code();
    dep_336::code_inlined();
    dep_336::code_generic(1u32);
}
