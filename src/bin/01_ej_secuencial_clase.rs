use std::io;

fn leer_numero_f64() -> f64 {
    let mut teclado = String::new();
    io::stdin()
        .read_line(&mut teclado)
        .expect("error en lectura");
    let numero: f64 = teclado
        .trim()
        .replace(',', ".")
        .parse()
        .expect("Número no válido");
    numero
}

fn main() {
    //   1.	Pedir por teclado base y altura de un rectángulo y calcular el perímetro y el área.
    //pedir la base
    println!("Base rectangulo");
    let base = leer_numero_f64();
    //pedir la altura
    println!("Altura del rectangulo");
    let altura = leer_numero_f64();

    //calculos
    // •	Perímetro = 2 * base + 2 * altura
    // •	Área = base * altura
    let perimetro = 2.0 * base + 2.0 * altura;
    let area = base * altura;
    //visualizo
    println!("Perimero: {}, area: {}", perimetro, area);

    //let mut teclado = String::new();
    // println!("Base rectangulo");
    // io::stdin()
    //     .read_line(&mut teclado)
    //     .expect("error en lectura");
    // //pasar de texto a numero
    // let base:f64 = teclado.trim()
    //     .parse()
    //     .expect("Número no válido");
    //teclado.clear();
    //pedir la altura
    // println!("Altura del rectangulo");
    // let altura = leer_numero_f64();
    // io::stdin()
    //     .read_line(&mut teclado)
    //     .expect("error en lectura");
    // //pasar de texto a numero
    // let altura:f64 = teclado.trim()
    //     .replace(',', ".")
    //     .parse()
    //     .expect("Número no válido");
    //
    //calculos
    // •	Perímetro = 2 * base + 2 * altura
    // •	Área = base * altura
    // let perimetro = 2.0 * base + 2.0 * altura;
    // let area = base * altura;
    // //visualizo
    // println!("Perimero: {}, area: {}",perimetro,area);

    // 2.	Leer las calificaciones de un alumno en 3 asignaturas, visualizar la nota media por pantalla

    println!("nota?: ");
    let mates = leer_numero_f64();
    println!("nota?: ");
    let lengua = leer_numero_f64();
    println!("nota?: ");
    let ciencias = leer_numero_f64();
    let media = (mates + lengua + ciencias) / 3.0;

    println!("Tu media de notas es: {:.2}", media);

    // 3.	Pedir dos números por teclado y visualizar el producto (n1 x n2)
    // 4.	Leer dos números y almacenarlos en dos variables, (ejemplo: n1 -> 5, n2 -> 7). Cambiar los valores. (en el ejemplo, n1 será 7 y n2 será 5)
    let mut n1 = leer_numero_f64();
    let mut n2 = leer_numero_f64();

    println!("Origen -> n1: {n1} - n2: {n2}");

    let aux = n1;
    n1 = n2;
    n2 = aux;

    println!("cambiados -> n1: {n1} - n2: {n2}");
}
