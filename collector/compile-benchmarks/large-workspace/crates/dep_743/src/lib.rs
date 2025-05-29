pub fn code() {
    println!("Hello from dep_743");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_743");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_743: {t}");
}

pub fn foo() {
    dep_395::code();
    dep_395::code_inlined();
    dep_395::code_generic(1u32);
    dep_398::code();
    dep_398::code_inlined();
    dep_398::code_generic(1u32);
}
