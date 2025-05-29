pub fn code() {
    println!("Hello from dep_893");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_893");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_893: {t}");
}

pub fn foo() {
    dep_392::code();
    dep_392::code_inlined();
    dep_392::code_generic(1u32);
    dep_395::code();
    dep_395::code_inlined();
    dep_395::code_generic(1u32);
    dep_304::code();
    dep_304::code_inlined();
    dep_304::code_generic(1u32);
    dep_375::code();
    dep_375::code_inlined();
    dep_375::code_generic(1u32);
}
