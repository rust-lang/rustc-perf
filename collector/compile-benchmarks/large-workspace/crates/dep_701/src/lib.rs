pub fn code() {
    println!("Hello from dep_701");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_701");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_701: {t}");
}

pub fn foo() {
    dep_408::code();
    dep_408::code_inlined();
    dep_408::code_generic(1u32);
    dep_383::code();
    dep_383::code_inlined();
    dep_383::code_generic(1u32);
    dep_294::code();
    dep_294::code_inlined();
    dep_294::code_generic(1u32);
    dep_342::code();
    dep_342::code_inlined();
    dep_342::code_generic(1u32);
    dep_247::code();
    dep_247::code_inlined();
    dep_247::code_generic(1u32);
}
