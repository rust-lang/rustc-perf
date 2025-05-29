pub fn code() {
    println!("Hello from dep_418");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_418");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_418: {t}");
}

pub fn foo() {
    dep_244::code();
    dep_244::code_inlined();
    dep_244::code_generic(1u32);
    dep_271::code();
    dep_271::code_inlined();
    dep_271::code_generic(1u32);
    dep_281::code();
    dep_281::code_inlined();
    dep_281::code_generic(1u32);
    dep_309::code();
    dep_309::code_inlined();
    dep_309::code_generic(1u32);
}
