fn insercion(v: &mut Vec<i32>) {
    let n = v.len();
    for i in 1..n{
        let mut j = i;
        let actual = v[i];
        while j > 0 && v[j-1] > actual {
            v[j] = v[j-1];
            j -= 1;
        }
        v[j] = actual;
    }
}

fn main() {
    let mut datos = vec![42, 7, 19, 3, 25, 1, 18];
    println!("Vector original:");
    println!("{:?}", datos);

    insercion(&mut datos);
    println!("Vector ordenado:");
    println!("{:?}", datos);
}
