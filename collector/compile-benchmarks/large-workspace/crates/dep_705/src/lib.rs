pub fn code() {
    println!("Hello from dep_705");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_705");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_705: {t}");
}

pub fn foo() {
    dep_367::code();
    dep_367::code_inlined();
    dep_367::code_generic(1u32);
    dep_229::code();
    dep_229::code_inlined();
    dep_229::code_generic(1u32);
    dep_400::code();
    dep_400::code_inlined();
    dep_400::code_generic(1u32);
    dep_274::code();
    dep_274::code_inlined();
    dep_274::code_generic(1u32);
    dep_335::code();
    dep_335::code_inlined();
    dep_335::code_generic(1u32);
}
