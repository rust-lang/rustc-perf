pub fn code() {
    println!("Hello from dep_641");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_641");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_641: {t}");
}

pub fn foo() {
    dep_246::code();
    dep_246::code_inlined();
    dep_246::code_generic(1u32);
    dep_161::code();
    dep_161::code_inlined();
    dep_161::code_generic(1u32);
    dep_196::code();
    dep_196::code_inlined();
    dep_196::code_generic(1u32);
}
