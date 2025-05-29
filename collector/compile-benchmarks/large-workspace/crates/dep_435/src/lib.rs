pub fn code() {
    println!("Hello from dep_435");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_435");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_435: {t}");
}

pub fn foo() {
    dep_308::code();
    dep_308::code_inlined();
    dep_308::code_generic(1u32);
    dep_391::code();
    dep_391::code_inlined();
    dep_391::code_generic(1u32);
    dep_267::code();
    dep_267::code_inlined();
    dep_267::code_generic(1u32);
    dep_324::code();
    dep_324::code_inlined();
    dep_324::code_generic(1u32);
}
