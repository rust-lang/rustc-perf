pub fn code() {
    println!("Hello from dep_616");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_616");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_616: {t}");
}

pub fn foo() {
    dep_335::code();
    dep_335::code_inlined();
    dep_335::code_generic(1u32);
    dep_294::code();
    dep_294::code_inlined();
    dep_294::code_generic(1u32);
    dep_303::code();
    dep_303::code_inlined();
    dep_303::code_generic(1u32);
    dep_305::code();
    dep_305::code_inlined();
    dep_305::code_generic(1u32);
}
