pub fn code() {
    println!("Hello from dep_291");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_291");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_291: {t}");
}

pub fn foo() {
    dep_70::code();
    dep_70::code_inlined();
    dep_70::code_generic(1u32);
    dep_157::code();
    dep_157::code_inlined();
    dep_157::code_generic(1u32);
    dep_92::code();
    dep_92::code_inlined();
    dep_92::code_generic(1u32);
}
