use std::io;

pub fn leer_numero_f32() -> f32 {
    let mut teclado = String::new();
    io::stdin().read_line(&mut teclado).expect("Error al leer");
    let numero: f32 = teclado.trim().replace(',', ".").parse().expect("Error al convertir a número");
    numero
}

pub fn leer_numero_i32(msg: &str) -> i32 {
    let teclado = leer_string(msg); 
    let numero = teclado
        .trim()
        .replace(',', ".")
        .parse()
        .expect("Número no válido");
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