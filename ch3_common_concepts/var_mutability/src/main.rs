fn main() {
    let x = 5;
    println!("x é {}", x);

    // x = 7;
    // println!("x é {}", x);
    println!("x é imutável, portanto seu valor não pode mudar\n");

    let mut y = 3;
    println!("y é {}", y);
    y = 10;
    println!("novo valor de y é {}\n", y);

    let length = 10;
    println!("tamanho é {}", length);
    let length = "Dez";
    println!("tamanho foi redeclarado, e agora é {}", length);
}
