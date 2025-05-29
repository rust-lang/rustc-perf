pub fn code() {
    println!("Hello from dep_555");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_555");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_555: {t}");
}

pub fn foo() {
    dep_192::code();
    dep_192::code_inlined();
    dep_192::code_generic(1u32);
    dep_317::code();
    dep_317::code_inlined();
    dep_317::code_generic(1u32);
    dep_331::code();
    dep_331::code_inlined();
    dep_331::code_generic(1u32);
    dep_325::code();
    dep_325::code_inlined();
    dep_325::code_generic(1u32);
}
