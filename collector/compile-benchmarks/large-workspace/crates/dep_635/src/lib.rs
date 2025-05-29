pub fn code() {
    println!("Hello from dep_635");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_635");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_635: {t}");
}

pub fn foo() {
    dep_282::code();
    dep_282::code_inlined();
    dep_282::code_generic(1u32);
    dep_268::code();
    dep_268::code_inlined();
    dep_268::code_generic(1u32);
    dep_306::code();
    dep_306::code_inlined();
    dep_306::code_generic(1u32);
    dep_246::code();
    dep_246::code_inlined();
    dep_246::code_generic(1u32);
    dep_263::code();
    dep_263::code_inlined();
    dep_263::code_generic(1u32);
}
