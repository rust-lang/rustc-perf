pub fn code() {
    println!("Hello from dep_732");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_732");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_732: {t}");
}

pub fn foo() {
    dep_203::code();
    dep_203::code_inlined();
    dep_203::code_generic(1u32);
    dep_405::code();
    dep_405::code_inlined();
    dep_405::code_generic(1u32);
    dep_380::code();
    dep_380::code_inlined();
    dep_380::code_generic(1u32);
    dep_303::code();
    dep_303::code_inlined();
    dep_303::code_generic(1u32);
    dep_197::code();
    dep_197::code_inlined();
    dep_197::code_generic(1u32);
}
