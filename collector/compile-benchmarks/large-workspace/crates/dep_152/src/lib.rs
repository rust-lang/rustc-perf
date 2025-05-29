pub fn code() {
    println!("Hello from dep_152");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_152");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_152: {t}");
}

pub fn foo() {
    dep_55::code();
    dep_55::code_inlined();
    dep_55::code_generic(1u32);
}
