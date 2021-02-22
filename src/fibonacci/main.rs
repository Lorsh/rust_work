use std::env;
use std::collections::HashMap;


fn fib(n: u8, memo: &mut HashMap<u8,u128>) -> u128 {
    if memo.get(&n) != None {return memo[&n]; }
    if  n <= 2 { return 1 };
    *memo.entry(n).or_insert(0) = fib(n-1,memo) + fib(n-2,memo);
    return memo[&n];
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let user_input = &args[1];
    let test = user_input.trim().parse::<u8>();
    match test {
        Err(e) => panic!("Not valid: {}.",e),
        _ => (),
    }
    let fib_number: u8 = user_input.parse().unwrap();
    if fib_number > 186 {
        panic!("integer is too large");
    }
    let mut map: HashMap<u8,u128> = HashMap::new();
    let result  = fib(fib_number,&mut map);
    println!("The {} fibonacci number is {}", fib_number,result);

}
