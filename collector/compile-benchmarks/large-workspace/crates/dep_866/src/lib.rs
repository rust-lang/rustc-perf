pub fn code() {
    println!("Hello from dep_866");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_866");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_866: {t}");
}

pub fn foo() {
    dep_161::code();
    dep_161::code_inlined();
    dep_161::code_generic(1u32);
    dep_306::code();
    dep_306::code_inlined();
    dep_306::code_generic(1u32);
}
