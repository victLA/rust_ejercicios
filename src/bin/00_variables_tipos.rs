#[allow(unused_variables, unused)]
fn main() {
    // 1. Declara una variable llamada edad e inicialízala con el valor 75.
    let edad = 75;

    // 2. Elige el tipo adecuado para esta variable: let serie_kennedy: ?? = 112263;.
    let serie_kennedy: u32 = 112263;

    // 3. Almacena en una variable la duración media de un episodio de la serie Perdidos (42 minutos).
    let duracion_perdidos: u8 = 42;

    // 4. Declara las variables dia_nacimiento, mes_nacimiento y anio_nacimiento, elige los tipos
    // adecuados (en base al conocimiento que tienes hasta ahora) y almacena en ellas tu fecha de
    // nacimiento.
    let dia_nacimiento: u8 = 16;
    let mes_nacimieto: u8 = 5;
    let mes_txt_nacimiento: &str = "mayo";
    let anio_nacimiento: u16 = 2020;

    // 5. En la serie Caleidoscopio, una banda de ladrones profesionales intenta forzar una caja fuerte
    // irrompible para hacerse con el mayor botín de la historia: siete mil millones de dólares.
    // Implementa una variable que almacene esta cantidad.
    let botin: u64 = 7_000_000_000;

    // 6. ¿Tienes el modo oscuro activado en tu navegador? Crea una variable booleana que almacene
    // tu respuesta (true si está activo, false si no lo está).
    let modo_color = false;

    // 7. La primera versión estable de Rust fue lanzada en el año 2015. Almacena este dato en una
    // constante.
    const RUST_UNO: u16 = 2015;

    // 8. Declara una variable llamada numero_de_puntos cuyo valor pueda cambiar a lo largo de la
    // ejecución de un programa.
    let mut numero_de_puntos = 0;

    // 9. El siguiente programa no funciona. Copia el código en tu editor de código favorito y haz que
    // compile.

    let mut edad: u8 = 30;
    let altura: f32 = 182.00;
    let inicial_nombre = 'E';
    edad = 32;
    let altura = 183;
    let modificador_caracter: &char = &inicial_nombre;
}
