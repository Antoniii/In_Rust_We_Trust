use std::io;

fn phi(mut x: i32) -> i32 {
	let mut result = x;
	
    for _i in 2..x {
    	if _i*_i <= x {
    		if x % _i == 0 {
    			while x % _i == 0 {
    				x /= _i;
    			}
    			result -= result / _i;
    		}
    	}
    }


	if x > 1 { result -= result / x; }

	return result;
}

fn main() {
	println!("Введите n:");

	let mut n = String::new();

	io::stdin().read_line(&mut n)
	    .ok()
	    .expect("Не удалось прочитать строку");

	let n: i32 = n.trim().parse()
	    .ok()
	    .expect("Введите число!");
	
	println!("Функция Эйлера phi({}) = {}", n, phi(n));
}