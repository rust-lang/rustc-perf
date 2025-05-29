pub fn code() {
    println!("Hello from dep_693");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_693");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_693: {t}");
}

pub fn foo() {
    dep_391::code();
    dep_391::code_inlined();
    dep_391::code_generic(1u32);
    dep_291::code();
    dep_291::code_inlined();
    dep_291::code_generic(1u32);
    dep_314::code();
    dep_314::code_inlined();
    dep_314::code_generic(1u32);
}
