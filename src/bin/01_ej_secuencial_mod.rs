//#[path = "../utilidades.rs"]
//mod utilidades;
//use utilidades::*;
//use utilidades::leer_numero_f32;
//use utilidades::{leer_numero, area_rectangulo};

use rust_ejercicios::utilidades::*;

fn main() {
    //     1.	Pedir por teclado base y altura de un rectángulo y calcular el perímetro y el área.
    // •	Perímetro=2 * base + 2 * altura
    // •	Área=base * altura
    println!("Base del rectángulo: ");
    let base = leer_numero_f32();

    println!("Altura del rectángulo: ");
    let altura = leer_numero_f32();

    let perimetro = 2.0 * base + 2.0 * altura;
    let area = base * altura;

    println!("Perímetro: {}", perimetro);
    println!("Área: {}", area);
}
