// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let mut answer = 3;
    square(&mut answer);
    println!("The square of 3 is {}", answer);
}

fn square(num:&mut i32)  {
    *num = (*num) * (*num);
}
