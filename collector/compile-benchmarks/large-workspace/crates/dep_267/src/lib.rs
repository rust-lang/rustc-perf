pub fn code() {
    println!("Hello from dep_267");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_267");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_267: {t}");
}

pub fn foo() {
    dep_91::code();
    dep_91::code_inlined();
    dep_91::code_generic(1u32);
    dep_132::code();
    dep_132::code_inlined();
    dep_132::code_generic(1u32);
    dep_114::code();
    dep_114::code_inlined();
    dep_114::code_generic(1u32);
}
