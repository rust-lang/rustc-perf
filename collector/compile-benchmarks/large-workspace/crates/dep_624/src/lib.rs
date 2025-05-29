pub fn code() {
    println!("Hello from dep_624");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_624");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_624: {t}");
}

pub fn foo() {
    dep_263::code();
    dep_263::code_inlined();
    dep_263::code_generic(1u32);
}
