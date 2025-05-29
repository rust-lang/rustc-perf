pub fn code() {
    println!("Hello from dep_598");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_598");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_598: {t}");
}

pub fn foo() {
    dep_194::code();
    dep_194::code_inlined();
    dep_194::code_generic(1u32);
    dep_342::code();
    dep_342::code_inlined();
    dep_342::code_generic(1u32);
}
