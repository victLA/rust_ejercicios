use std::io;
fn main() {
    //     1.	Pedir por teclado base y altura de un rectángulo y calcular el perímetro y el área.
    // •	Perímetro=2 * base + 2 * altura
    // •	Área=base * altura
    println!("Base del rectángulo: ");
    //leer
    let mut teclado = String::new();
    io::stdin().read_line(&mut teclado).expect("Error al leer");
    //parsear

    let base = teclado
        .trim()
        .parse::<f32>()
        .expect("Error al convertir a número");
    //limpiar
    teclado.clear();

   // let base = leer_numero_f32();
   // let altura = leer_numero_f32();
   
    println!("Altura del rectángulo: ");
    io::stdin().read_line(&mut teclado).expect("Error al leer");
    let altura: f32 = teclado.trim().replace(',', ".").parse().expect("Error al convertir a número");

    let perimetro = 2.0 * base + 2.0 * altura;
    let area = base * altura;

    println!("Perímetro: {}", perimetro);    
    println!("Área: {}", area); 

    // 2.	Leer las calificaciones de un alumno en 3 asignaturas, visualizar la nota media por pantalla
    // 3.	Pedir dos números por teclado y visualizar el producto (n1 x n2)
    // 4.	Leer dos números y almacenarlos en dos variables, (ejemplo: n1 -> 5, n2 -> 7). Cambiar los valores. (en el ejemplo, n1 será 7 y n2 será 5)
}

// fn leer_numero_f32() -> f32 {
//     let mut teclado = String::new();
//     io::stdin().read_line(&mut teclado).expect("Error al leer");
//     let numero: f32 = teclado.trim().replace(',', ".").parse().expect("Error al convertir a número");
//     numero
// }
