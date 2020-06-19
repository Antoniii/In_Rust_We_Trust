use std::io;

fn nod(x: i64, y: i64) -> i64 {
    if y != 0 { return nod(y, x % y); } 
    else { return x; }
}	

fn main() {
	println!("Введите n1:");
	let mut n1 = String::new();
    
    io::stdin().read_line(&mut n1)
	    .ok()
	    .expect("Не удалось прочитать строку");

	let n1: i64 = n1.trim().parse()
	     .ok()
	     .expect("Введите число!");


    println!("Введите n2:");
	let mut n2 = String::new();

	io::stdin().read_line(&mut n2)
	    .ok()
	    .expect("Не удалось прочитать строку");

	let n2: i64 = n2.trim().parse()
	     .ok()
	     .expect("Введите число!");
	
	println!("НОД = {}", nod(n1,n2));
	println!("НОК = {}", (n1*n2)/nod(n1,n2));
}