pub fn code() {
    println!("Hello from dep_777");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_777");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_777: {t}");
}

pub fn foo() {
    dep_380::code();
    dep_380::code_inlined();
    dep_380::code_generic(1u32);
}
