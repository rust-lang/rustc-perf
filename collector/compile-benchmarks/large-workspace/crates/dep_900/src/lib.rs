pub fn code() {
    println!("Hello from dep_900");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_900");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_900: {t}");
}

pub fn foo() {
    dep_280::code();
    dep_280::code_inlined();
    dep_280::code_generic(1u32);
    dep_291::code();
    dep_291::code_inlined();
    dep_291::code_generic(1u32);
    dep_390::code();
    dep_390::code_inlined();
    dep_390::code_generic(1u32);
}
