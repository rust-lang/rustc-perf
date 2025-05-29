pub fn code() {
    println!("Hello from dep_105");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_105");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_105: {t}");
}

pub fn foo() {
    dep_15::code();
    dep_15::code_inlined();
    dep_15::code_generic(1u32);
    dep_48::code();
    dep_48::code_inlined();
    dep_48::code_generic(1u32);
}
