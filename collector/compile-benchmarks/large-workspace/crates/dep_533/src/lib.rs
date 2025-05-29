pub fn code() {
    println!("Hello from dep_533");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_533");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_533: {t}");
}

pub fn foo() {
    dep_371::code();
    dep_371::code_inlined();
    dep_371::code_generic(1u32);
    dep_231::code();
    dep_231::code_inlined();
    dep_231::code_generic(1u32);
    dep_201::code();
    dep_201::code_inlined();
    dep_201::code_generic(1u32);
}
