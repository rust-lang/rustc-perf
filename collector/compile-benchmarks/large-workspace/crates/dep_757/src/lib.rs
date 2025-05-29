pub fn code() {
    println!("Hello from dep_757");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_757");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_757: {t}");
}

pub fn foo() {
    dep_381::code();
    dep_381::code_inlined();
    dep_381::code_generic(1u32);
    dep_367::code();
    dep_367::code_inlined();
    dep_367::code_generic(1u32);
}
