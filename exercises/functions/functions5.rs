// functions5.rs
// Make me compile! Execute `rustlings hint functions5` for hints :)

// I AM DONE

fn main() {
    let answer = square(3);
    println!("The answer is {}", answer);
}

fn square(num: i32) -> i64 {
    return (num * num).into();
}
