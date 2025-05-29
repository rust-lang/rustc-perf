pub fn code() {
    println!("Hello from dep_544");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_544");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_544: {t}");
}

pub fn foo() {
    dep_168::code();
    dep_168::code_inlined();
    dep_168::code_generic(1u32);
    dep_344::code();
    dep_344::code_inlined();
    dep_344::code_generic(1u32);
}
