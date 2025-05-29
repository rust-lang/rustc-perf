pub fn code() {
    println!("Hello from dep_496");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_496");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_496: {t}");
}

pub fn foo() {
    dep_404::code();
    dep_404::code_inlined();
    dep_404::code_generic(1u32);
}
