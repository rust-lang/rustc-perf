pub fn code() {
    println!("Hello from dep_812");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_812");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_812: {t}");
}

pub fn foo() {
    dep_288::code();
    dep_288::code_inlined();
    dep_288::code_generic(1u32);
    dep_342::code();
    dep_342::code_inlined();
    dep_342::code_generic(1u32);
    dep_332::code();
    dep_332::code_inlined();
    dep_332::code_generic(1u32);
}
