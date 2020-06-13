use std::io;

fn fact(x: i64) -> i64 {
    if x >= 1 {
    	return x*fact(x-1);
    } 
    else {
    	return 1;
    }
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
	
	println!("{}! = {}", n, fact(n));
}

// max: 20! = 2432902008176640000