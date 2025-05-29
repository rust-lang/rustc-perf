pub fn code() {
    println!("Hello from dep_844");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_844");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_844: {t}");
}

pub fn foo() {
    dep_210::code();
    dep_210::code_inlined();
    dep_210::code_generic(1u32);
    dep_234::code();
    dep_234::code_inlined();
    dep_234::code_generic(1u32);
    dep_238::code();
    dep_238::code_inlined();
    dep_238::code_generic(1u32);
    dep_222::code();
    dep_222::code_inlined();
    dep_222::code_generic(1u32);
    dep_397::code();
    dep_397::code_inlined();
    dep_397::code_generic(1u32);
}
