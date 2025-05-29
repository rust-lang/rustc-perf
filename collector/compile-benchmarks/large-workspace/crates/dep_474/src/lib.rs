pub fn code() {
    println!("Hello from dep_474");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_474");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_474: {t}");
}

pub fn foo() {
    dep_388::code();
    dep_388::code_inlined();
    dep_388::code_generic(1u32);
    dep_315::code();
    dep_315::code_inlined();
    dep_315::code_generic(1u32);
    dep_187::code();
    dep_187::code_inlined();
    dep_187::code_generic(1u32);
    dep_303::code();
    dep_303::code_inlined();
    dep_303::code_generic(1u32);
    dep_196::code();
    dep_196::code_inlined();
    dep_196::code_generic(1u32);
}
