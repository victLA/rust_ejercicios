use rust_ejercicios::utilidades::*;
use std::io;

pub fn read_f64(input: &mut String) -> f64 {
    let n: f64;
    loop {
        input.clear();
        io::stdin().read_line(input).expect("Error al leer");
        match input.trim().replace(',', ".").parse() {
            Ok(num) => {
                n = num;
                break;
            }
            Err(_e) => {
                println!("Introduce un número");
            }
        }
    }
    n
}

fn consulta_saldo(saldo: f64) -> f64 {
    println!("Tu saldo actual es {}€", saldo);
    saldo
}

fn main() {
    let mut saldo = 1000.0;
    loop {
        println!("\n--- Opciones cajero ---");
        println!("1. Consultar saldo");
        println!("2. Ingresar");
        println!("3. Retirar efectivo");
        println!("4. Salir");
        println!("Seleccionar una opcion: ");

        //leer opción
        let op = leer_string("Opciones posibles (1, 2 , 3 , 4)");
        let op = op.as_str();

        //realizar operación según la opción elegida
        match op {
            "1" => {
                consulta_saldo(saldo);
            }
            "2" => {
                //Ingresar
                let cantidad = leer_numero_f64("¿Cuanto quieres ingresar?");
                if cantidad > 0.0 {
                    saldo += cantidad;
                } else {
                    println!("Cantidad no valida,  el valor debe ser mayor que 0");
                }

                // match leer_numero_f64_option("¿Cuanto quieres ingresar? ") {
                //     Some(cantidad) if cantidad > 0.0 => {
                //         saldo += cantidad;
                //         consulta_saldo(saldo);
                //     }
                //     Some(_) => println!("No puedes ingresar un valor negativo o cero"),
                //     None => println!("Introduce un número válido"),
                // }
            }
            "3" => {
                let cantidad = leer_numero_f64("¿Cuanto quieres sacar?");
                if cantidad <= saldo {
                    saldo -= saldo;
                    consulta_saldo(saldo);
                }else if cantidad <= 0.0 {
                    println!("El valor no pueder negativo o 0");
                }else{
                    println!("Cantidad no valida,  saldo insuficiente");
                }                
                // match leer_numero_f64_option("¿Cuánta cantidad quieres sacar? ") {
                //     Some(cantidad) if cantidad <= 0.0 => {
                //         println!("No puedes retirar un valor negativo o cero");
                //     }
                //     Some(cantidad) if cantidad > saldo => {
                //         println!("No tienes suficiente saldo");
                //     }
                //     Some(cantidad) => {
                //         saldo -= cantidad;
                //         println!("El nuevo saldo es: {:.2}", saldo);
                //     }
                //     None => println!("Introduce un número válido"),
                // }
            }
            "4" => {
                println!("Hasta la próxima");
                break;
            }
            _ => {
                println!("Opción no válida");
            }
        }
    }
}
