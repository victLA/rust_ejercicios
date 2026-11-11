#![allow(unused)]
use std::io;

fn leer_numero_i32() -> i32 {
    let mut teclado = String::new();
    println!("Introducir número");
    io::stdin()
        .read_line(&mut teclado)
        .expect("error en lectura");
    let numero = teclado
        .trim()
        .replace(',', ".")
        .parse()
        .expect("Número no válido");
    numero
}

fn leer_string(msg: &str) -> String {
    println!("{msg}");
    let mut teclado = String::new();
    io::stdin()
        .read_line(&mut teclado)
        .expect("error en lectura");
    let caracter = teclado.trim().to_string();
    caracter
}

fn leer_char() -> char {
    let texto = leer_string("Operador (+, -, *, /): ");
    let mut chars = texto.chars(); //devuelve un iterador sobre los caracteres unicode
    let c = chars.next();
    match c {
        Some(dato) => dato,
        None => panic!("error"),
    }
}


fn main() {
    // 1. Recibo por teclado 2 números y un operador que podrá ser (+, - , *, /) realizar la operación
    // que corresponda y calcular el resultado.
    let n1 = leer_numero_i32();
    let n2 = leer_numero_i32();
    /*
    leer_string devuelve un String (un valor con dueño, guardado en el heap),
    si quiero un &str (una referencia a texto almacenada en stack) hay que tomar una referencia al String con &.
    */
    //let operacion: &str = &leer_string("Operador (+, -, *, /): ");
    // ------------------ opción devolver un char -> leer_char()
    let ope = leer_char();
    println!("{}", ope);
    //
    let total = match ope {
        '+' => n1 + n2,
        '-' => n1 - n2,
        '*' => n1 * n2,
        '/' => {
            if n2 == 0 {
                0
            } else {
                n1 / n2
            }
        }
        _ => panic!("operador no válido"),
    };
    println!("El resultado con leer_char() es {}", total);

    // --------- opción devolver un String y pasar a &str con la función as_str() -> leer_string(msg: &str)
    let operacion = leer_string("Operador (+, -, *, /): ");
    let total = match operacion.as_str() {
        "+" => n1 + n2,
        "-" => n1 - n2,
        "*" => n1 * n2,
        "/" => n1 / n2,
        _ => panic!("operador no válido"),
    };
    println!("El resultado con leer_string() es {}", total);

    //let mut total = 0;
    // if operacion == "+"{
    //     total = n1 + n2;
    // }else if  operacion == "-"{
    //     total = n1 - n2;
    // }else if operacion == "*" {
    //     total = n1 * n2;
    // }else if operacion == "/" {
    //     total = n1 / n2;
    // }else {
    //     println!("Operación no válida");
    // }
    // println!("El resultado es {}", total);

    // 2. Introducir un número y visualizar si es par o impar.

    // 3. Introducir un número y visualizar si es positivo o negativo (0 es positivo).

    // 4. Almacenar en una variable un número y pedir al usuario que escriba un número, si coincide
    // con el introducido le indicamos acierto y si no, error.
    // 5. Elaborar un programa que muestre el precio de un billete de autobús, que se calcula en base
    // a los kilómetros de trayecto (0.30 Euros por Km), pero si el recorrido supera los 80 Km se aplica
    // un 15 % de descuento y que si el trayecto se realiza en día laborable (‘L’) hay un 5 % de
    // descuento respecto si es día festivo (‘F’). Datos que introducir: kilómetros de recorrido y tipo
    // de día (L-F).
    // 6. Dado el sueldo bruto de un individuo (pedir por teclado), calcular su pensión teniendo en
    // cuenta que esta será el 80% del sueldo bruto, pero que como máximo podrá cobrar 2300 €
    // 7. Realiza el siguiente menú de un cajero automático
}
