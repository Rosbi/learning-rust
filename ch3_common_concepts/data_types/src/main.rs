fn main() {
    let unsigned_8: u8 = 255;
    let arch: usize = 0;
    let int: i32 = 0xff;
    let long_num = 999_999_999;

    println!("Inteiros podem ser unsigned(u) ou signed(i)");
    println!("    e podem ter os tamanhos 8, 16, 32, 64");
    println!("    exemplos: u16, i64\n");

    println!("unsigned_8: {}", unsigned_8);
    println!("arch: {}", arch);
    println!("int: {}", int);
    println!("long_num: {}\n", long_num);

    println!("Podem ser floats também, mas apenas 32 ou 64 bits");
    println!("    f32 | f64\n");

    let character: char = 'ℤ';
    println!("Chars são UTF-8, não ASCII. Símbolo inteiro: {}\n", character);

    // =================== Tipos Compostos ========================

    println!("Rust tem tuplas! NATIVAMENTE!!");
    let tup: (i32, f64) = (1974, 15.80);
    println!("tup.0: {}", tup.0);
    println!("tup.1: {}", tup.1);

    let (x, y) = tup;
    println!("Elas também podem ser decompostas:");
    println!("x: {}", x);
    println!("y: {}\n", y);

    let _array1 = [1, 2, 3]; // array de 3 elementos
    let _array2: [i64; 10];  // array vazio com 10 elementos, todos i64
    let _array3 = [15; 7];   // array com 7 elementos, todos inicializados com valor 15

    let array_of_tuples: [(f32, char); 2] = [
        (15.2, 'B'),
        (62.75, 'C')
    ];
    println!("Vetor de tuplas:");
    println!("array 0 tupla 1: {}", array_of_tuples[0].1);
    println!("array 1 tupla 0: {}\n", array_of_tuples[1].0);

    let tuple_of_arrays: ([i32; 3], [f64; 2], [char; 4]) = (
        [1, 2, 3],
        [4.1, 4.2],
        ['A', 'B', 'C', 'D']
    );
    println!("Tupla de vetores:");
    println!("tupla 0 vetor 2: {}", tuple_of_arrays.0[2]);
    println!("tupla 1 vetor 0: {}", tuple_of_arrays.1[0]);
    println!("tupla 2 vetor 4: {}", tuple_of_arrays.2[3]);
}
