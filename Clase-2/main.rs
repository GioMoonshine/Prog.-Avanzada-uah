fn main() {
    let temp:[f32;7] = [25.3, 19.5, 23.1, 13.9, 12.6, 11.7, 30.8];
    let mut max:f32 = 0.0;
    let mut min:f32 = 0.0;
    let mut prom:f32 = 0.0;
    let mut aux:f32 = 0.0;
    let days:Vec<&str> = vec!["Lunedi", "Martedi", "Mercoledi", "Giovedi", "Venerdi", "Sabato", "Domenica"];
    let mut dayssel:Vec<&str> = vec![];

    for i in 0..temp.len() {
        if temp[i] > max {
            max = temp[i];
        }
        if min == 0.0 {
            min = temp[i];
        } else if min != 0.0 && temp[i] < min {
            min = temp[i];
        }
        prom += temp[i];
        aux += 1.0;
    }
    prom /= aux;
    for i in 0..temp.len() {
        if temp[i] > prom {
            dayssel.push(days[i]);
        }
    }

    println!("La temperatura máxima es: {:?}", max);
    println!("La temperatura mínima es: {:?}", min);
    println!("El promedio de temperatura es: {:?}", prom);
    let mut cty:i32 = 0;
    for _i in 0..dayssel.len() {
        cty += 1;
    }
    if cty == 1 {
        println!("El día que supera la temperatura promedio es: {:?}", dayssel);
    } else {
        println!("Los días que superan la temperatura promedio son: {:?}", dayssel);
    }
}
