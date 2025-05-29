pub fn code() {
    println!("Hello from dep_191");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_191");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_191: {t}");
}

pub fn foo() {
    dep_67::code();
    dep_67::code_inlined();
    dep_67::code_generic(1u32);
    dep_116::code();
    dep_116::code_inlined();
    dep_116::code_generic(1u32);
    dep_72::code();
    dep_72::code_inlined();
    dep_72::code_generic(1u32);
}
