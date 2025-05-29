pub fn code() {
    println!("Hello from dep_333");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_333");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_333: {t}");
}

pub fn foo() {
    dep_63::code();
    dep_63::code_inlined();
    dep_63::code_generic(1u32);
    dep_126::code();
    dep_126::code_inlined();
    dep_126::code_generic(1u32);
}
