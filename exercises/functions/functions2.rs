// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.


const abcdefg:i8=7;
fn main() {
    call_me(3);
}

fn call_me(num:i8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
