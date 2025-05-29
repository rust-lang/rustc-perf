pub fn code() {
    println!("Hello from dep_643");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_643");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_643: {t}");
}

pub fn foo() {
    dep_361::code();
    dep_361::code_inlined();
    dep_361::code_generic(1u32);
    dep_326::code();
    dep_326::code_inlined();
    dep_326::code_generic(1u32);
}
