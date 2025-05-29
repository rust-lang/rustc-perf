pub fn code() {
    println!("Hello from dep_130");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_130");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_130: {t}");
}

pub fn foo() {
    dep_15::code();
    dep_15::code_inlined();
    dep_15::code_generic(1u32);
    dep_45::code();
    dep_45::code_inlined();
    dep_45::code_generic(1u32);
}
