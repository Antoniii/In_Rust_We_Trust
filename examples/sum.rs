use std::io;

fn sum(x: i64) -> i64 {
    if x != 0 { return x + sum(x-1); } 
    else { return x; }
}	

fn main() {
	println!("Введите n:");

	let mut n = String::new();

	io::stdin().read_line(&mut n)
	    .ok()
	    .expect("Не удалось прочитать строку");

	let n: i64 = n.trim().parse()
	    .ok()
	    .expect("Введите число!");
	
	println!("Sum = {}", sum(n));
}