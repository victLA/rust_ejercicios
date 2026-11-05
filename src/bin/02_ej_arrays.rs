#![allow(unused)]

use rand::RngExt;
use std::io;

fn leer_numero_i32() -> i32 {
    let mut teclado = String::new();
    io::stdin()
        .read_line(&mut teclado)
        .expect("error en lectura");
    let numero: i32 = teclado
        .trim()
        .replace(',', ".")
        .parse()
        .expect("Número no válido");
    numero
}
fn main() {
    // 1.	Rellenar un array de 5 números de tipo entero.Una vez relleno, pedir un número y contestar al usuario si ese número está en el array.
    let mut array = [0; 5]; // array de 5 enteros inicializado a 0

    for mut num in &mut array {
        *num = rand::rng().random_range(1..=10); // rellenar el array con números aleatorios del 1 al 10
    }
    println!("{:?}", array);

    println!("Introduce un número a buscar en el array: ");
    let buscado = leer_numero_i32();

    let mut encontrado = false;
    for num in array {
        // let mut encontrado = false;
        if buscado == num {
            encontrado = true;
        }
    }
    println!("El número está en el array: {}", encontrado);

    // 2.	Programa para comprar asientos de un autobús de 10 plazas,
    // este marca los asientos ocupados con un “1” y los libres con un “0”.
    // Solicitar al comprador que plaza quiere (del 1 al 10) y si el asiento ya se ha vendido, el programa lo señalará,
    // si no se ha vendido la plaza quedará marcada con un “1” y le deseamos “Feliz viaje”.
    let mut asientos = [0u8; 10];
    for mut num in &mut asientos {
        *num = rand::rng().random_range(0..2); // rellenar el array con números aleatorios 0 o 1
    }
    println!("{:?}", asientos);

    // 3.	Almacenar 300 números aleatorios entre -20 y 20 en un array, imprimir cuantos son ceros, cuantos son negativos, cuantos positivos.
 

}
