
//use std::io;

fn is_prime(x:i32) ->bool{
    //try using sieve of erastosthenes of sieve of atkin even better instead here
    for i in 2..x/2+x%2+1{
        if x%i==0{
            return false
        }

    }
    return true
}

fn main() {
    // add automated test cases make it grade itself of something
    println!("Hello, world!");
    let mut num=100;

    for i in 1..num{
        //println!("{i}");
        if is_prime(i){
            println!("{i} is prime!")
        }
    }


    //io::stdin()
        //.read_line(&mut num)
        //.expect("Failed to read line");

    println!("x");

}
