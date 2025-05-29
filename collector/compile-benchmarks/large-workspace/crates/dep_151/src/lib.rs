pub fn code() {
    println!("Hello from dep_151");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_151");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_151: {t}");
}

pub fn foo() {
    dep_43::code();
    dep_43::code_inlined();
    dep_43::code_generic(1u32);
    dep_20::code();
    dep_20::code_inlined();
    dep_20::code_generic(1u32);
    dep_51::code();
    dep_51::code_inlined();
    dep_51::code_generic(1u32);
    dep_55::code();
    dep_55::code_inlined();
    dep_55::code_generic(1u32);
}
