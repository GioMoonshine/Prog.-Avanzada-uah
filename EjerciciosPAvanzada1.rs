fn main() {
	/*
	Ejercicio 1
	Conteo de números pares
	*/
	println!("Problema 1");
	let list1 = vec![3, 8, 2, 7, 10, 5, 4];								//Establecemos la lista
	let mut ctdad = 0;													//Asignamos el conteo como mutable
	for i in 0..list1.len() {											//Recorremos la lista mediante un bucle "for"
		if list1[i] % 2 == 0 {											//Comparamos el resto del elemento en la posición "i" de la lista
			ctdad += 1;													//Si el resto del valor da 0, aumenta en 1 la cantidad de valores pares
		}
	}
	println!("La cantidad de números pares que hay es de: {}", ctdad);	//Imprimimos para verificar si el código está correcto

	/*
	Ejercicio 2
	Suma de valores positivos
	*/
	println!("Problema 2");	
	let list2 = vec![-3, 5, -2, 7, -1, 4];										//Se establece la lista
	let mut suma = 0;															//Asignamos la variable de suma como mutable
	for i in 0..list2.len() {													//Recorremos la lista utilizando un bucle "for"
		if list2[i] >= 0 {														//Comparamos los valores de la lista con un "if"
			suma += list2[i];													//Si el valor de la lista es mayor o igual a 0, se suma a la variable de suma
		}
	}
	println!("La suma de los valores positivos de la lista es de {}", suma);	//Se realiza la impresión para verificar el código

	/*
	Ejercicio 3
	Encontrar el valor máximo
	*/
	println!("Problema 3");
	let list3 = vec![12, 5, 8, 21, 3, 17];						//Se establece la lista
	let mut compare = 0;										//Asignamos una variable mutable para comparación como mutable
	for i in 0..list3.len() {									//Se recorre la lista utilizando un bucle "for"
		if list3[i] > compare {									//Comparamos el valor de la variable de comparación con cada valor de la lista
			compare = list3[i];									//Si el valor es mayor, se asigna ese valor a la variable de comparación
		}
	}
	println!("El valor máximo de la lista es: {}", compare);	//Se imprime para verificar el código

	/*
	Ejercicio 4
	Promedio de notas
	*/
	println!("Problema 4");
	let list4 = vec![5.0, 4.5, 6.2, 5.8, 3.9];			//Se establece la lista(importante establecerlo como float)
	let mut prom = 0.0;									//Asignamos la variable de promedio(igualmente importante el float)
	let mut total = 0.0;								//Asignamos la variable para división(float)
	for i in 0..list4.len() {							//Con bucle "for" recorremos la lista
		prom += list4[i];								//Cada vez que se recorre la lista, sumamos cada valor
		total += 1.0;									//Sumamos cada vez que se recorre la lista para saber la cantidad de valores que hay
	}
	prom = prom/total;									//Realizamos la división utilizando la suma de los valores y el total de valores
	println!("El promedio de notas es de: {}", prom);	//Se imprime para verificar el código

	/*
	Ejercicio 5
	Conteo de valores mayores que un umbral
	*/
	println!("Problema 5");
	let arreglo = vec![7, 12, 5, 18, 9];												//Establecemos los valores del arreglo en una lista
	let t = 10;																			//Asignamos el umbral T
	let mut ctdad = 0;																	//Creamos una variable mutable para contar la cantidad de valores que son mayores que el umbral
	for i in 0..arreglo.len() {															//Se recorre la lista para acceder a cada valor de esta
		if arreglo[i] > t {																//Comparamos los valores de la lista con el umbral
			ctdad += 1;																	//Por cada valor mayor que el umbral, aumentamos la variable de cantidad
		}
	}
	println!("La cantidad de valores del arreglo mayores a T es igual a: {}", ctdad);	//Verificamos el funcionamiento del código

	/*
	Ejercicio 6
	Búsqueda de un elemento
	*/
	println!("Problema 6");
	let arreglo = vec![4, 9, 2, 7, 6];							//Establecemos los valores del arreglo en una lista
	let x = 7;													//Asignamos una variable con el valor deseado
	let mut esta = false;										//Asignamos una variable booleana para declarar si el valor está o no
	for i in 0..arreglo.len() {									//Recorremos la lista para acceder a sus valores
		if arreglo[i] == x {									//Comparamos los valores de la lista con la variable con el valor a buscar
			esta = true;										//Si se encuentra el valor en la lista, cambiamos la variable booleana para indicar que si está
		}
	}
	if esta == true {
		println!("El valor está presente en el arreglo");		//Imprimimos que el valor está si la variable booleana es verdadera
	} else {
		println!("El valor no está presente en el arreglo");	//Caso contrario, imprime que el valor no está
	}

	/*
	Ejercicio 7
	Conteo de repeticiones
	*/
	println!("Problema 7");
	let arreglo = vec![2, 4, 2, 5, 2, 7, 3];					//
	let x = 2;													//
	let mut ctdad = 0;											//
	for i in 0..arreglo.len() {									//
		if arreglo[i] == x {									//
			ctdad += 1;											//
		}
	}
	if ctdad == 0 {
		println!("El número {} no aparece en la lista", x);		//
	} else if ctdad == 1 {
		println!("El número {} aparece {} vez", x, ctdad);		//
	} else if ctdad >= 1 {
		println!("El número {} aparece {} veces", x, ctdad);	//
	}

	/*
	Ejercicio 8
	Identificación de valores consecutivos
	*/
	println!("Problema 8");
	let arreglo = vec![4, 7, 7, 3, 9];												//
	let mut compare = 0;															//
	let mut verify = false;															//
	for i in 0..arreglo.len() {														//
		if arreglo[i] == compare {													//
			verify = true;															//
		}
		compare = arreglo[i]														//
	}
	if verify == true {
		println!("Si existen dos números consecutivos iguales dentro del arreglo");	//
	} else {
		println!("No existen dos números consecutivos iguales dentro del arreglo");	//
	}

	/*
	Ejercicio 9
	Longitud de la secuencia creciente
	*/
	println!("Problema 9");
	let arreglo = vec![2, 3, 5, 1, 4, 6, 7];															//
	let mut compare = -10;																				//
	let mut lengh = 1;																					//
	let mut maxlengh = 0;																				//
	for i in 0..arreglo.len() {																			//
		if arreglo[i] == compare+1 {																	//
			lengh += 1;																					//
		} else {
			if maxlengh < lengh {																		//
				maxlengh = lengh																		//
			}
			lengh = 1;																					//
		}
		compare = arreglo[i];																			//
	}
	println!("La longitud de subsecuencia creciente más larga dentro del arreglo es de: {}", maxlengh);	//

	/*
	Ejercicio 10
	Detección de valores atípicos
	*/
	println!("Problema 10");
	let arreglo = vec![18, 19, 21, 22 ,35, 20, 19];									//
	let mut prom = 0;																//
	let mut valores = vec![];														//
	let mut total = 0;																//
	for i in 0..arreglo.len() {														//
		prom += arreglo[i];															//
		total += 1;																	//
	}
	prom /= total;																	//
	for i in 0..arreglo.len() {														//
		if arreglo[i] >= prom+10 {													//
			valores.push(arreglo[i]);												//
		}
	}
	println!("El promedio de temperaturas es de: {}", prom);						//
	println!("Las temperaturas {:?} superan en 10 grados el promedio", valores);	//
}
//Pude haber hecho funciones, pero quería retomar el hábito de escribir.