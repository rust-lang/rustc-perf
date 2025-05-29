pub fn code() {
    println!("Hello from dep_392");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_392");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_392: {t}");
}

pub fn foo() {
    dep_143::code();
    dep_143::code_inlined();
    dep_143::code_generic(1u32);
    dep_99::code();
    dep_99::code_inlined();
    dep_99::code_generic(1u32);
    dep_93::code();
    dep_93::code_inlined();
    dep_93::code_generic(1u32);
    dep_77::code();
    dep_77::code_inlined();
    dep_77::code_generic(1u32);
}
