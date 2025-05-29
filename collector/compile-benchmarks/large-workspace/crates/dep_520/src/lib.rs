pub fn code() {
    println!("Hello from dep_520");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_520");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_520: {t}");
}

pub fn foo() {
    dep_263::code();
    dep_263::code_inlined();
    dep_263::code_generic(1u32);
    dep_180::code();
    dep_180::code_inlined();
    dep_180::code_generic(1u32);
    dep_161::code();
    dep_161::code_inlined();
    dep_161::code_generic(1u32);
    dep_274::code();
    dep_274::code_inlined();
    dep_274::code_generic(1u32);
}
