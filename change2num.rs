fn main() {
    //let (A,B) = (4,5);
    
    let mut x: i32 = 4;
    let mut y = 5;

    println!("До перестановки А = {}\n", x);
    println!("B = {}\n", y);
    
    x = x - y;
	y = x + y;
	x = y - x;

	println!("После перестановки А = {}\n", x);
    println!("B = {}\n", y);
}