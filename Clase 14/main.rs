use std::time::{Duration, Instant};

fn buscar(v: &Vec<i32>, objetivo: i32) -> Option<usize> {
    for i in 0..v.len() {
        if v[i] == objetivo {
            return Some(i);
        }
    }
    None
}

fn main() {
    let tamanos = vec![
    1_000,
    10_000,
    100_000,
    1_000_000,
    10_000_000
    ];
    for n in tamanos {
        //let n = 10_000_000;
        let mut datos = Vec::new();
        for i in 0..n {
            datos.push(i as i32);
        }

        let objetivo = datos[n-1];

        let mut tiempo_total = Duration::new(0, 0);
        let repeticiones = 10;

        for i in 0..repeticiones {
            let inicio = Instant::now();
            let resultado = buscar(&datos, objetivo);
            let duracion = inicio.elapsed();
            //println!("Tiempo de búsqueda: {:?}", duracion);
            tiempo_total += duracion;
        }
        
        let promedio = tiempo_total/repeticiones;

        println!("Promedio de tiempo del tamano {:?}: {:?}", n, promedio);
    }
}