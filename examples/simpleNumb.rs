fn main() {
	let (mut x, y) = (1, 100);
	let mut flag;
    
    while x < y {

    	flag = 0;
        
        for _i in 2..x/2 {
		
			if x % _i == 0 {
				flag = 1;
				break;
			}
		}

		if flag == 0 { print!("{} ", x);}

		x += 1;
    }
    println!();
}