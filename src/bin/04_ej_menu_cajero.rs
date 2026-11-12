
use rust_ejercicios::utilidades::*;

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
