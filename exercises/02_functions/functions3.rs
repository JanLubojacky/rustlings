fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    // TODO: Fix the function call.
    let num: u32 = 20;
    call_me(num);
}
