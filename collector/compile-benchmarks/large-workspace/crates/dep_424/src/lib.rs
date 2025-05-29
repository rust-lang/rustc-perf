pub fn code() {
    println!("Hello from dep_424");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_424");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_424: {t}");
}

pub fn foo() {
    dep_264::code();
    dep_264::code_inlined();
    dep_264::code_generic(1u32);
    dep_187::code();
    dep_187::code_inlined();
    dep_187::code_generic(1u32);
    dep_201::code();
    dep_201::code_inlined();
    dep_201::code_generic(1u32);
    dep_168::code();
    dep_168::code_inlined();
    dep_168::code_generic(1u32);
    dep_307::code();
    dep_307::code_inlined();
    dep_307::code_generic(1u32);
}
