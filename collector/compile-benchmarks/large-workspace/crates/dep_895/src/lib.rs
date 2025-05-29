pub fn code() {
    println!("Hello from dep_895");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_895");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_895: {t}");
}

pub fn foo() {
    dep_347::code();
    dep_347::code_inlined();
    dep_347::code_generic(1u32);
    dep_224::code();
    dep_224::code_inlined();
    dep_224::code_generic(1u32);
    dep_167::code();
    dep_167::code_inlined();
    dep_167::code_generic(1u32);
    dep_236::code();
    dep_236::code_inlined();
    dep_236::code_generic(1u32);
}
