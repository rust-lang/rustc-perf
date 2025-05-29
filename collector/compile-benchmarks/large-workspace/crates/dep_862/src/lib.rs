pub fn code() {
    println!("Hello from dep_862");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_862");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_862: {t}");
}

pub fn foo() {
    dep_335::code();
    dep_335::code_inlined();
    dep_335::code_generic(1u32);
    dep_328::code();
    dep_328::code_inlined();
    dep_328::code_generic(1u32);
    dep_333::code();
    dep_333::code_inlined();
    dep_333::code_generic(1u32);
}
