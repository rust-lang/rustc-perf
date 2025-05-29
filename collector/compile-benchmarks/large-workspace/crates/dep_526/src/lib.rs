pub fn code() {
    println!("Hello from dep_526");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_526");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_526: {t}");
}

pub fn foo() {
    dep_354::code();
    dep_354::code_inlined();
    dep_354::code_generic(1u32);
    dep_286::code();
    dep_286::code_inlined();
    dep_286::code_generic(1u32);
}
