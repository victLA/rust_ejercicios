use rust_ejercicios::utilidades::*;
use std::fmt;

// ---------- Errores ----------
#[derive(Debug)]
enum ErrorCuenta {
    CantidadNoValida,
    SaldoInsuficiente,
    LimiteRetiradas(u32),
}

impl fmt::Display for ErrorCuenta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorCuenta::CantidadNoValida => write!(f, "La cantidad debe ser mayor que 0"),
            ErrorCuenta::SaldoInsuficiente => write!(f, "Saldo insuficiente"),
            ErrorCuenta::LimiteRetiradas(n) => {
                write!(f, "Has alcanzado el límite de {n} retiradas este mes")
            }
        }
    }
}

fn validar_cantidad(cantidad: f64) -> Result<(), ErrorCuenta> {
    if cantidad > 0.0 {
        Ok(())
    } else {
        Err(ErrorCuenta::CantidadNoValida)
    }
}

// ---------- Trait ----------
// Trait con métodos obligatorios y por defecto:
// disponible() y fin_de_mes() tienen implementación por defecto.
// CuentaCorriente sobrescribe disponible() y CuentaAhorro sobrescribe fin_de_mes()
trait Cuenta {
    // Métodos obligatorios: cada cuenta los implementa a su manera
    fn tipo(&self) -> &str;
    fn saldo(&self) -> f64;
    fn ingresar(&mut self, cantidad: f64) -> Result<(), ErrorCuenta>;
    fn retirar(&mut self, cantidad: f64) -> Result<(), ErrorCuenta>;

    // Métodos con implementación por defecto: se pueden sobrescribir
    fn disponible(&self) -> f64 {
        self.saldo()
    }

    fn fin_de_mes(&mut self) {
        // por defecto no hace nada
    }
}

// ---------- Cuenta corriente (con descubierto) ----------
struct CuentaCorriente {
    saldo: f64,
    descubierto_max: f64,
}

impl CuentaCorriente {
    fn new(saldo: f64, descubierto_max: f64) -> Self {
        Self {
            saldo,
            descubierto_max,
        }
    }
}

impl Cuenta for CuentaCorriente {
    fn tipo(&self) -> &str {
        "Cuenta corriente"
    }

    fn saldo(&self) -> f64 {
        self.saldo
    }

    fn disponible(&self) -> f64 {
        self.saldo + self.descubierto_max
    }
    // () ? -> devuelve el error automáticamente si falla, así que no repites el if en cada método
    fn ingresar(&mut self, cantidad: f64) -> Result<(), ErrorCuenta> {
        validar_cantidad(cantidad)?;
        self.saldo += cantidad;
        Ok(())
    }

    fn retirar(&mut self, cantidad: f64) -> Result<(), ErrorCuenta> {
        validar_cantidad(cantidad)?;
        if cantidad > self.disponible() {
            return Err(ErrorCuenta::SaldoInsuficiente);
        }
        self.saldo -= cantidad; // el saldo puede quedar negativo
        Ok(())
    }
}

// ---------- Cuenta de ahorro (intereses y límite de retiradas) ----------
struct CuentaAhorro {
    saldo: f64,
    interes_anual: f64, // 0.02 = 2 %
    retiradas_mes: u32,
    max_retiradas: u32,
}

impl CuentaAhorro {
    fn new(saldo: f64, interes_anual: f64, max_retiradas: u32) -> Self {
        Self {
            saldo,
            interes_anual,
            retiradas_mes: 0,
            max_retiradas,
        }
    }
}

impl Cuenta for CuentaAhorro {
    fn tipo(&self) -> &str {
        "Cuenta de ahorro"
    }

    fn saldo(&self) -> f64 {
        self.saldo
    }

    fn ingresar(&mut self, cantidad: f64) -> Result<(), ErrorCuenta> {
        validar_cantidad(cantidad)?;
        self.saldo += cantidad;
        Ok(())
    }

    fn retirar(&mut self, cantidad: f64) -> Result<(), ErrorCuenta> {
        validar_cantidad(cantidad)?;
        if self.retiradas_mes >= self.max_retiradas {
            return Err(ErrorCuenta::LimiteRetiradas(self.max_retiradas));
        }
        if cantidad > self.saldo {
            return Err(ErrorCuenta::SaldoInsuficiente);
        }
        self.saldo -= cantidad;
        self.retiradas_mes += 1;
        Ok(())
    }

    fn fin_de_mes(&mut self) {
        let interes = self.saldo * self.interes_anual / 12.0;
        self.saldo += interes;
        self.retiradas_mes = 0;
        println!("Intereses abonados: {:.2}€", interes);
    }
}

// ---------- Cajero ----------
struct Cajero {
    cuenta: Box<dyn Cuenta>, // cualquier tipo que implemente Cuenta
}
impl Cajero {
    /*
    Box<dyn Cuenta>: es un trait object.
    El tamaño de cada struct es distinto, así que hay que ponerlos en el heap detrás de un puntero.
    El coste es una llamada indirecta (vtable)
    */
    fn new(cuenta: Box<dyn Cuenta>) -> Self {
        Self { cuenta }
    }

    fn mostrar_menu(&self) {
        println!("\n--- Cajero: {} ---", self.cuenta.tipo());
        println!("1. Consultar saldo");
        println!("2. Ingresar");
        println!("3. Retirar efectivo");
        println!("4. Simular fin de mes");
        println!("5. Salir");
    }

    fn consultar_saldo(&self) {
        println!("Saldo actual: {:.2}€", self.cuenta.saldo());
        if self.cuenta.disponible() != self.cuenta.saldo() {
            println!("Disponible:   {:.2}€", self.cuenta.disponible());
        }
    }

    fn ingresar(&mut self) {
        match leer_numero_f64_option("¿Cuánto quieres ingresar?") {
            Some(c) => match self.cuenta.ingresar(c) {
                Ok(()) => self.consultar_saldo(),
                Err(e) => println!("{e}"),
            },
            None => println!("Introduce un número válido"),
        }
    }

    fn retirar(&mut self) {
        match leer_numero_f64_option("¿Cuánto quieres sacar?") {
            Some(c) => match self.cuenta.retirar(c) {
                Ok(()) => self.consultar_saldo(),
                Err(e) => println!("{e}"),
            },
            None => println!("Introduce un número válido"),
        }
    }

    fn ejecutar(&mut self) {
        loop {
            self.mostrar_menu();
            match leer_string("Selecciona una opción (1-5):").as_str() {
                "1" => self.consultar_saldo(),
                "2" => self.ingresar(),
                "3" => self.retirar(),
                "4" => {
                    self.cuenta.fin_de_mes();
                    self.consultar_saldo();
                }
                "5" => {
                    println!("Hasta la próxima");
                    break;
                }
                _ => println!("Opción no válida"),
            }
        }
    }
}

fn elegir_cuenta() -> Box<dyn Cuenta> {
    loop {
        println!("Tipo de cuenta:");
        println!("1. Corriente (descubierto de 500€)");
        println!("2. Ahorro (2 % anual, 3 retiradas al mes)");
        match leer_string("Elige (1 o 2):").as_str() {
            "1" => return Box::new(CuentaCorriente::new(1000.0, 500.0)),
            "2" => return Box::new(CuentaAhorro::new(1000.0, 0.02, 3)),
            _ => println!("Opción no válida"),
        }
    }
}
// genéricos (static dispatch) hay que usarla en compilación, 
// aquí, donde el usuario elige el tipo en ejecución, dyn es la opción adecuada
// struct Cajero<C: Cuenta> {
//     cuenta: C,
// }

fn main() {
    let mut cajero = Cajero::new(elegir_cuenta());
    cajero.ejecutar();
}
