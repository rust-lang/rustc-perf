pub fn code() {
    println!("Hello from dep_59");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_59");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_59: {t}");
}

pub fn foo() {
    dep_9::code();
    dep_9::code_inlined();
    dep_9::code_generic(1u32);
}
