pub fn code() {
    println!("Hello from dep_629");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_629");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_629: {t}");
}

pub fn foo() {
    dep_281::code();
    dep_281::code_inlined();
    dep_281::code_generic(1u32);
    dep_263::code();
    dep_263::code_inlined();
    dep_263::code_generic(1u32);
}
