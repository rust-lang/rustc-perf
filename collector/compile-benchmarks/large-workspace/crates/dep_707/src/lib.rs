pub fn code() {
    println!("Hello from dep_707");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_707");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_707: {t}");
}

pub fn foo() {
    dep_162::code();
    dep_162::code_inlined();
    dep_162::code_generic(1u32);
}
