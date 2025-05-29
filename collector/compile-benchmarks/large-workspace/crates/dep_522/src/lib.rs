pub fn code() {
    println!("Hello from dep_522");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_522");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_522: {t}");
}

pub fn foo() {
    dep_161::code();
    dep_161::code_inlined();
    dep_161::code_generic(1u32);
}
