pub fn code() {
    println!("Hello from dep_758");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_758");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_758: {t}");
}

pub fn foo() {
    dep_246::code();
    dep_246::code_inlined();
    dep_246::code_generic(1u32);
    dep_409::code();
    dep_409::code_inlined();
    dep_409::code_generic(1u32);
    dep_261::code();
    dep_261::code_inlined();
    dep_261::code_generic(1u32);
    dep_335::code();
    dep_335::code_inlined();
    dep_335::code_generic(1u32);
}
