fn main() {
	let mut tx_izq = vec![];
	let mut tx_der = vec![];
	/*
	add("P", tx_izq);
	add("r", tx_izq);
	add("o", tx_izq);
	add("g", tx_izq);
	add("r", tx_izq);
	add("a", tx_izq);
	*/

	tx_izq.push("p");
	tx_izq.push("r");
	tx_izq.push("o");
	tx_izq.push("g");
	tx_izq.push("r");
	tx_izq.push("a");

	println!("{:?}", tx_izq);

	let tx = tx_izq.pop();
	tx_der.insert(0, tx.expect("REASON"));
	println!("{:?} | {:?}", tx_izq, tx_der);

	let tx = tx_izq.pop();
	tx_der.insert(0, tx.expect("REASON"));
	println!("{:?} | {:?}", tx_izq, tx_der);}

/*
fn add(c:&str, tx_izq:Vec<str>) {
	tx_izq.clone().push(&c);
}
*/