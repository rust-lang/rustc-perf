pub fn code() {
    println!("Hello from dep_761");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_761");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_761: {t}");
}

pub fn foo() {
    dep_338::code();
    dep_338::code_inlined();
    dep_338::code_generic(1u32);
    dep_261::code();
    dep_261::code_inlined();
    dep_261::code_generic(1u32);
    dep_361::code();
    dep_361::code_inlined();
    dep_361::code_generic(1u32);
    dep_220::code();
    dep_220::code_inlined();
    dep_220::code_generic(1u32);
}
