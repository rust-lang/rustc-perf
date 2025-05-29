pub fn code() {
    println!("Hello from dep_709");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_709");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_709: {t}");
}

pub fn foo() {
    dep_371::code();
    dep_371::code_inlined();
    dep_371::code_generic(1u32);
    dep_189::code();
    dep_189::code_inlined();
    dep_189::code_generic(1u32);
    dep_273::code();
    dep_273::code_inlined();
    dep_273::code_generic(1u32);
    dep_303::code();
    dep_303::code_inlined();
    dep_303::code_generic(1u32);
    dep_267::code();
    dep_267::code_inlined();
    dep_267::code_generic(1u32);
}
