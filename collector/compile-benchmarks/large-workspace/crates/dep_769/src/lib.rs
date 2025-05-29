pub fn code() {
    println!("Hello from dep_769");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_769");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_769: {t}");
}

pub fn foo() {
    dep_402::code();
    dep_402::code_inlined();
    dep_402::code_generic(1u32);
    dep_232::code();
    dep_232::code_inlined();
    dep_232::code_generic(1u32);
    dep_367::code();
    dep_367::code_inlined();
    dep_367::code_generic(1u32);
    dep_370::code();
    dep_370::code_inlined();
    dep_370::code_generic(1u32);
}
