pub fn code() {
    println!("Hello from dep_606");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_606");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_606: {t}");
}

pub fn foo() {
    dep_303::code();
    dep_303::code_inlined();
    dep_303::code_generic(1u32);
    dep_339::code();
    dep_339::code_inlined();
    dep_339::code_generic(1u32);
    dep_222::code();
    dep_222::code_inlined();
    dep_222::code_generic(1u32);
}
