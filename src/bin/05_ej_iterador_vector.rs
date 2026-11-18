use rand::RngExt;

fn main() {
    let mut numeros = Vec::new();

    for i in 1..=300 {
        let valor = rand::rng().random_range(-10..=10);
        numeros.push(valor);
    }
    //println!("{:?}", numeros);

    let num: Vec<i32> = (0..300).map(|_| rand::random_range(-10..10)).collect();
    //println!("{:?}", num);

    let ceros = numeros.iter().filter(|n| **n == 0).count(); // o filter(|&&n| n == 0)
    let positivos = numeros.iter().filter(|n| **n > 0).count(); // o numeros.iter().copied().filter(|&n| n == 0).count();
    let negativos = numeros.iter().filter(|n| **n < 0).count();

    println!(
        "Positivos: {}, Negativos: {} y Ceros: {}. en total: {} números",
        positivos,
        negativos,
        ceros,
        numeros.len()
    );

    let (negativos, ceros, positivos) =
        numeros
            .iter()
            .fold((0usize, 0usize, 0usize), |(n, c, p), &x| {
                if x < 0 {
                    (n + 1, c, p)
                } else if x == 0 {
                    (n, c + 1, p)
                } else {
                    (n, c, p + 1)
                }
            });
}
