use std::io;

pub fn leer_numero_f32() -> f32 {
    let mut teclado = String::new();
    io::stdin().read_line(&mut teclado).expect("Error al leer");
    let numero: f32 = teclado.trim().replace(',', ".").parse().expect("Error al convertir a número");
    numero
}