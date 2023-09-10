
#[link(name = "adder", kind="static")]
extern "C"{
    fn add(a:i32, b: i32) -> i32;
}
use std::arch::asm;

static mut COUNTER: i32 = 0;

fn increment_counter(){
    unsafe{
        let z = add(1,1);
        add_r(COUNTER, z);
    }    
}

fn add_r(x: i32, y: i32) -> i32 {
    let result: i32;

    unsafe {
        // x86/x86-64 assembly
        asm!("add {0}, {1}", inout(reg) x => result, in (reg) y);
    }

    result
}

fn main() {
    let mut s = "Let's Get Rusty".to_owned();

    let raw1 = &s as *const String;
    let raw2 = &mut s as *mut String;

    let address = 0x012345usize;
    let raw3 = address as *const String;

    unsafe{
        (*raw2).push_str("!!!");
        println!("raw1 is {}", *raw1);

        my_function();
    }

    for _ in 0..10 {
        increment_counter();
    }

    unsafe{
        println!("COUNTER: {}", COUNTER);
    }    
}

unsafe fn my_function(){
    println!("Calling my function!");
}

unsafe trait MyTrait{
    fn some_function(&self);
}

unsafe impl MyTrait for String{
    fn some_function(&self) {
        //...
    }
}