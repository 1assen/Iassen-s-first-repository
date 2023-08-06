fn is_prime(x:i32) ->bool{
    if x<=1{
        return false;
    }

    let numbers:Vec<i32>=(2..=((x as f32).sqrt().round()) as i32).collect();
    let result:Vec<i32>=numbers
        .iter()
        .filter(|&num| x % num == 0)
        .copied()
        .collect();
    return result.len()==0
}


        
    


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn it_works(){
        assert_eq!(is_prime(5),true);
        assert_eq!(is_prime(6),false);
        assert_eq!(is_prime(7),true);
        assert_eq!(is_prime(8),false);
    }

    #[test]
    fn failure(){
        //just exploring what failed tests look like
        panic!("this one is supposed to fail")
    }

    #[test]
    fn failure2(){
        assert_eq!(is_prime(5),false);
    }  
}



fn erastosthenes_prime_list(x:i32)->Vec<bool>{
    //generates a list of primes up to the number x using sieve of erastosthenes method
    //no for loops allowed!
    /* 
    let mut counter:i32=0;
    */let primes_list: Vec<bool>=vec![];/* 
    //automatically assume all numbers within our list up to x are prime (true)
    //primes_list[0:x]=false;
    //figure out better way to do this
    loop{
        if counter>=x{
            break;
        }
        primes_list.push(true);
        counter+=1;
    }


    //make all non prime composite numbers within our list false
    counter=2;
    let xf32:f32=x as f32;
    loop{

        //
        if counter>xf32.sqrt().round() as i32{
            break;
        }

        let mut inner_counter:i32=counter;
        loop{
            if inner_counter>=x{
                break
            }
            primes_list[inner_counter]=false;
            inner_counter+=counter;
        }
    }
     */
    return primes_list;
    

}

fn main() {
    println!("Hello, world!");
    let num=100;

    for i in 1..num{
        //println!("{i}");
        if is_prime(i){
            println!("{i} is prime!")
        }
    }
}
