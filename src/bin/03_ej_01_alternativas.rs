#![allow(unused)]
use rand::RngExt;
use std::io;

fn leer_numero_i32(msg: &str) -> i32 {
    let teclado = leer_string(msg);

    let numero: i32 = match teclado.trim().replace(',', ".").parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Error de conversión");
            return 0;
        }
    };
    // .expect("Número no válido");
    numero
}

fn leer_numero_f64(msg: &str) -> f64 {
    let teclado = leer_string(msg);
    let numero: f64 = match teclado.trim().replace(',', ".").parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Error de conversión");
            return 0.0;
        }
    };
    // .expect("Número no válido");
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
    //let n1 = leer_numero_i32("Introducir número");

    //let n2 = leer_numero_i32("Introducir número");
    /*
    leer_string devuelve un String (un valor con dueño, guardado en el heap),
    si quiero un &str (una referencia a texto almacenada en stack) hay que tomar una referencia al String con &.
    */
    //let operacion: &str = &leer_string("Operador (+, -, *, /): ");

    // // ------------------ opción devolver un char -> leer_char()
    // let ope = leer_char();
    // println!("{}", ope);
    // //
    // let total = match ope {
    //     '+' => n1 + n2,
    //     '-' => n1 - n2,
    //     '*' => n1 * n2,
    //     '/' => {
    //         if n2 == 0 {
    //             0
    //         } else {
    //             n1 / n2
    //         }
    //     }
    //     _ => panic!("operador no válido"),
    // };
    // println!("El resultado con leer_char() es {}", total);

    // // --------- opción devolver un String y pasar a &str con la función as_str() -> leer_string(msg: &str)
    // let operacion = leer_string("Operador (+, -, *, /): ");
    // let op_txt = operacion.as_str();
    // let total = match op_txt {
    //     "+" => n1 + n2,
    //     "-" => n1 - n2,
    //     "*" => n1 * n2,
    //     "/" => n1 / n2,
    //     _ => panic!("operador no válido"),
    // };
    // println!("El resultado con leer_string() es {}", total);

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

    //pedir el numero
    //comparar con % 2
    //si es = 0  -> par
    //si no -> impar
    // let numero = leer_numero_i32("Dime un numerito");
    // let resultado = if numero % 2 == 0 { "par" } else { "impar" };
    // println!("{resultado}");
    // println!(
    //     "El numero es: {}",
    //     if numero % 2 == 0 { "par" } else { "impar" }
    // );

    // 3. Introducir un número y visualizar si es positivo o negativo (0 es positivo).
    // let numero = leer_numero_i32("Dime un numerito");
    // println!(
    //     "El numero es: {}",
    //     if numero >= 0 { "positivo" } else { "negattivo" }
    // );

    // 4. Almacenar en una variable un número y pedir al usuario que escriba un número, si coincide
    // con el introducido le indicamos acierto y si no, error.
    //guardar un numero en variable
    //pedir un numero
    //comparar si son iguales
    let num_secreto = rand::rng().random_range(1..10);
    let num_user = leer_numero_i32("¡Adivina un número!");
    let acierto = if num_secreto == num_user {
        "Ok, Acierto!"
    } else {
        "No has acertado!!"
    };
    println!("{acierto}");

    // 5. Elaborar un programa que muestre el precio de un billete de autobús, que se calcula en base
    // a los kilómetros de trayecto (0.30 Euros por Km), pero si el recorrido supera los 80 Km se aplica
    // un 15 % de descuento y que si el trayecto se realiza en día laborable (‘L’) hay un 5 % de
    // descuento respecto si es día festivo (‘F’). Datos que introducir: kilómetros de recorrido y tipo
    // de día (L-F).
    // const PRECIO_KM: f64 = 0.30;
    // const LIMITE_KM: f64 = 80.0;
    // const DESCUENTO_DIA: f64 = 0.05;
    // const DESCUENTO_DISTANCIA: f64 = 0.15;

    // let km = leer_numero_f64("Introducir kilometros");
    // let dia = leer_string("¿Qué dia viajas? (L = Laborable, F = fnestivo)");

    // let mut precio_billete = km * PRECIO_KM;

    // let mut dto_km = if km > LIMITE_KM {
    //     precio_billete * DESCUENTO_DISTANCIA
    // } else {
    //     0.0
    // };

    // let dto_dia = match dia.trim().to_uppercase().as_str() {
    //     "L" => precio_billete * DESCUENTO_DIA,
    //     "F" => 0.0,
    //     _ => panic!("Día no válido"),
    // };

    // let mut dtos = 0.0;
    // if km > LIMITE_KM {
    //     dtos += DESCUENTO_DISTANCIA;
    // }
    // if dia == "L" {
    //     dtos += DESCUENTO_DIA;
    // }

    // precio_billete -= dto_dia - dto_km;

    /*  6. Dado el sueldo bruto de un individuo (pedir por teclado), 
    calcular su pensión teniendo en cuenta que esta será el 80% del sueldo bruto, 
    pero que como máximo podrá cobrar 2300 €*/

    //crear las constantes
    //pedir el sueldo bruto
    //calcular la pensión
    //si pensión es mayor que > 2300
    // corregir asignar maximo
    //visualizar pensión
    const PENSION_MAX: f64 = 2300.0;
    const PORCENTAJE_PENSION: f64 = 0.80;
    const NUM_PAGAS: f64 = 14.0;

    let sueldo_mes = leer_numero_f64("Sueldo mensual");
  
    let mut pension = sueldo_mes * PORCENTAJE_PENSION;

      println!("la pension que corresponde es {pension}");

    if (pension > PENSION_MAX) {
        pension = PENSION_MAX;
    }

    println!("Su pensiòn mensual será {pension} y su pensión anual {}", pension * NUM_PAGAS);

    // 7. Realiza el siguiente menú de un cajero automático

    
}
