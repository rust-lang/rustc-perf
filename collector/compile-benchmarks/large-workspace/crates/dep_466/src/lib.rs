pub fn code() {
    println!("Hello from dep_466");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_466");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_466: {t}");
}

pub fn foo() {
    dep_395::code();
    dep_395::code_inlined();
    dep_395::code_generic(1u32);
    dep_261::code();
    dep_261::code_inlined();
    dep_261::code_generic(1u32);
    dep_357::code();
    dep_357::code_inlined();
    dep_357::code_generic(1u32);
    dep_216::code();
    dep_216::code_inlined();
    dep_216::code_generic(1u32);
    dep_210::code();
    dep_210::code_inlined();
    dep_210::code_generic(1u32);
}
