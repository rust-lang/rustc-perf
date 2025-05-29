pub fn code() {
    println!("Hello from dep_145");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_145");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_145: {t}");
}

pub fn foo() {
    dep_15::code();
    dep_15::code_inlined();
    dep_15::code_generic(1u32);
    dep_35::code();
    dep_35::code_inlined();
    dep_35::code_generic(1u32);
}
