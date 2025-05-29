pub fn code() {
    println!("Hello from dep_668");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_668");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_668: {t}");
}

pub fn foo() {
    dep_328::code();
    dep_328::code_inlined();
    dep_328::code_generic(1u32);
    dep_286::code();
    dep_286::code_inlined();
    dep_286::code_generic(1u32);
}
