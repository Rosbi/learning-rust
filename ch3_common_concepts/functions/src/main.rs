fn increment_x(x: i32) -> i32 {
    x + 1
}

fn main() {
    println!("Uma expressão sem ';' é um valor retornado");

    let x = 5;
    println!("x incrementado: {}", increment_x(x));
}
