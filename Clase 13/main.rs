fn invertir_arreglo(v: &mut Vec<i32>) -> Vec<i32> {
    let mut arr = Vec::new();
    let mut n = v.len();

    while n > 0 {
        n -= 1;
        arr.push(v[n]);
    }
    arr
}

fn inv_arreglo2(v: &mut Vec<i32>) -> Vec<i32> {
    let mut izq = 0;
    let mut der = v.len()-1;
    while izq < der{
        let aux = v[der];
        v[der] = v[izq];
        v[izq] = aux;
        izq += 1;
        der -= 1;
    }
    v.to_vec()
}

fn main() {
    let mut arreglo = vec![3,1,8,5,6];
    arreglo = invertir_arreglo(&mut arreglo);
    println!("{:?}", arreglo);
    arreglo = inv_arreglo2(&mut arreglo);
    println!("{:?}", arreglo);
}
