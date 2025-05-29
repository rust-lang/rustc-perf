pub fn code() {
    println!("Hello from dep_335");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_335");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_335: {t}");
}

pub fn foo() {
    dep_130::code();
    dep_130::code_inlined();
    dep_130::code_generic(1u32);
    dep_93::code();
    dep_93::code_inlined();
    dep_93::code_generic(1u32);
    dep_96::code();
    dep_96::code_inlined();
    dep_96::code_generic(1u32);
}
