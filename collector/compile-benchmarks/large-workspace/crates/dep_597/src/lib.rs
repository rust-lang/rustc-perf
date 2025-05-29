pub fn code() {
    println!("Hello from dep_597");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_597");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_597: {t}");
}

pub fn foo() {
    dep_364::code();
    dep_364::code_inlined();
    dep_364::code_generic(1u32);
    dep_267::code();
    dep_267::code_inlined();
    dep_267::code_generic(1u32);
    dep_347::code();
    dep_347::code_inlined();
    dep_347::code_generic(1u32);
    dep_343::code();
    dep_343::code_inlined();
    dep_343::code_generic(1u32);
    dep_395::code();
    dep_395::code_inlined();
    dep_395::code_generic(1u32);
}
