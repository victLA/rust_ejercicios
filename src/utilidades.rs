use std::io::{self, Write};
use std::str::FromStr;

pub fn leer_numero_f32(msg: &str) -> f32 {
    let teclado = leer_string(msg);
    let numero: f32 = match teclado.trim().replace(',', ".").parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Error de conversión");
            return 0.0;
        }
    };
    numero
}

pub fn leer_numero_i32(msg: &str) -> i32 {
    let teclado = leer_string(msg);
    let numero = match teclado.trim().replace(',', ".").parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Error de conversión");
            return 0;
        }
    };
    // .expect("Número no válido");
    numero
}

pub fn leer_numero_f64(msg: &str) -> f64 {

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

pub fn leer_string(msg: &str) -> String {
    println!("{msg}");
    let mut teclado = String::new();
    io::stdin()
        .read_line(&mut teclado)
        .expect("error en lectura");
    let texto = teclado.trim().to_string();
    texto
}

pub fn leer_char() -> char {
    let texto = leer_string("Operador (+, -, *, /): ");
    let mut chars = texto.chars(); //devuelve un iterador sobre los caracteres unicode
    let c = chars.next();
    match c {
        Some(dato) => dato,
        None => panic!("error"),
    }
}

pub fn pide_dato<T: std::str::FromStr>(texto: &str, error: &str, validador: Option<fn(&T) -> bool>) -> T
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let mut entrada: String = String::new();
    print!("{}", texto);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut entrada)
        .expect("Error al leer la linea");

    let  valor;

    if let Some(primer_caracter) = entrada.trim().chars().next() {
        if primer_caracter.is_numeric() {
            valor = entrada.trim().replace(",", ".").parse::<T>().expect(error);
        } else {
            valor = entrada.trim().parse::<T>().expect(error);
        }
    } else {
        panic!("{}", error);
    }

    if let Some(validar) = validador {
        if !validar(&valor) {
            panic!("{}", error);
        }
    }
    valor
}
