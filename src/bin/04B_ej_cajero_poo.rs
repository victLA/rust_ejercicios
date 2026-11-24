use rust_ejercicios::utilidades::*;
use std::fmt;

// ---------- Errores del dominio ----------
#[derive(Debug)]
enum ErrorCuenta {
    CantidadNoValida,
    SaldoInsuficiente,
}

impl fmt::Display for ErrorCuenta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorCuenta::CantidadNoValida => write!(f, "La cantidad debe ser mayor que 0"),
            ErrorCuenta::SaldoInsuficiente => write!(f, "Saldo insuficiente"),
        }
    }
}

// ---------- Cuenta bancaria ----------
struct CuentaBancaria {
    saldo: f64, // privado: solo se modifica mediante métodos
}

impl CuentaBancaria {
    fn new(saldo_inicial: f64) -> Self {
        Self { saldo: saldo_inicial }
    }

    fn saldo(&self) -> f64 {
        self.saldo
    }

    fn ingresar(&mut self, cantidad: f64) -> Result<(), ErrorCuenta> {
        if cantidad <= 0.0 {
            return Err(ErrorCuenta::CantidadNoValida);
        }
        self.saldo += cantidad;
        Ok(())
    }

    fn retirar(&mut self, cantidad: f64) -> Result<(), ErrorCuenta> {
        if cantidad <= 0.0 {
            return Err(ErrorCuenta::CantidadNoValida);
        }
        if cantidad > self.saldo {
            return Err(ErrorCuenta::SaldoInsuficiente);
        }
        self.saldo -= cantidad;
        Ok(())
    }
}

// ---------- Cajero (interfaz de usuario) ----------
struct Cajero {
    cuenta: CuentaBancaria,
}

impl Cajero {
    fn new(cuenta: CuentaBancaria) -> Self {
        Self { cuenta }
    }

    fn mostrar_menu(&self) {
        println!("\n--- Opciones cajero ---");
        println!("1. Consultar saldo");
        println!("2. Ingresar");
        println!("3. Retirar efectivo");
        println!("4. Salir");
    }

    fn consultar_saldo(&self) {
        println!("Tu saldo actual es {:.2}€", self.cuenta.saldo());
    }

    fn ingresar(&mut self) {
        match leer_numero_f64_option("¿Cuánto quieres ingresar?") {
            Some(cantidad) => match self.cuenta.ingresar(cantidad) {
                Ok(()) => self.consultar_saldo(),
                Err(e) => println!("{e}"),
            },
            None => println!("Introduce un número válido"),
        }
    }

    fn retirar(&mut self) {
        match leer_numero_f64_option("¿Cuánto quieres sacar?") {
            Some(cantidad) => match self.cuenta.retirar(cantidad) {
                Ok(()) => self.consultar_saldo(),
                Err(e) => println!("{e}"),
            },
            None => println!("Introduce un número válido"),
        }
    }

    /// Bucle principal; devuelve cuando el usuario elige salir.
    fn ejecutar(&mut self) {
        loop {
            self.mostrar_menu();
            let op = leer_string("Selecciona una opción (1, 2, 3, 4):");

            match op.as_str() {
                "1" => self.consultar_saldo(),
                "2" => self.ingresar(),
                "3" => self.retirar(),
                "4" => {
                    println!("Hasta la próxima");
                    break;
                }
                _ => println!("Opción no válida"),
            }
        }
    }
}


fn main() {
    let cuenta = CuentaBancaria::new(1000.0);
    let mut cajero = Cajero::new(cuenta);
    cajero.ejecutar();
}


