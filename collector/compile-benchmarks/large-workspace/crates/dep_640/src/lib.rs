pub fn code() {
    println!("Hello from dep_640");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_640");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_640: {t}");
}

pub fn foo() {
    dep_384::code();
    dep_384::code_inlined();
    dep_384::code_generic(1u32);
    dep_210::code();
    dep_210::code_inlined();
    dep_210::code_generic(1u32);
    dep_297::code();
    dep_297::code_inlined();
    dep_297::code_generic(1u32);
    dep_239::code();
    dep_239::code_inlined();
    dep_239::code_generic(1u32);
}
