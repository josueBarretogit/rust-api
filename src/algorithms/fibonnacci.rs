use std::io;

fn calc_fibonacci() {

    let mut input = String::new(); 

    println!("Input a number to calculate fibonnaci");

    io::stdin().read_line(&mut  input)
        .expect("Did not input");


    let input : i64 = match input.trim().parse() {
       Ok(num) => num,
       Err(_e) => 1
    };

    println!("The fibonnacci of: {} is {}", input, fibonnacci_rust(input));
}

fn fibonnacci_rust(n : i64) -> i64 {
    if n == 0 {
        return n;
    }
    return fibonnacci_rust(n - 1) + fibonnacci_rust(n - 2)
}
