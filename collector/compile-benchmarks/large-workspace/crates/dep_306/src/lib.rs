pub fn code() {
    println!("Hello from dep_306");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_306");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_306: {t}");
}

pub fn foo() {
    dep_62::code();
    dep_62::code_inlined();
    dep_62::code_generic(1u32);
    dep_95::code();
    dep_95::code_inlined();
    dep_95::code_generic(1u32);
    dep_112::code();
    dep_112::code_inlined();
    dep_112::code_generic(1u32);
    dep_119::code();
    dep_119::code_inlined();
    dep_119::code_generic(1u32);
}
