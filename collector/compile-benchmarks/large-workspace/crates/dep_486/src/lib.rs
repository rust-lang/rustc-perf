pub fn code() {
    println!("Hello from dep_486");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_486");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_486: {t}");
}

pub fn foo() {
    dep_342::code();
    dep_342::code_inlined();
    dep_342::code_generic(1u32);
    dep_179::code();
    dep_179::code_inlined();
    dep_179::code_generic(1u32);
    dep_302::code();
    dep_302::code_inlined();
    dep_302::code_generic(1u32);
    dep_388::code();
    dep_388::code_inlined();
    dep_388::code_generic(1u32);
}
