pub fn code() {
    println!("Hello from dep_541");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_541");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_541: {t}");
}

pub fn foo() {
    dep_263::code();
    dep_263::code_inlined();
    dep_263::code_generic(1u32);
}
