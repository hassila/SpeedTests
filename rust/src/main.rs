const N: i32 = 440_000_000;


fn is_munchausen(number: i32, cache: &[i32; 10]) -> bool
{
    let mut n = number;
    let mut total = 0;

    while n > 0
    {
        let digit = n % 10;
        total += cache[digit as usize];
        if total > number {
            return false;
        }
        n = n / 10;
    }

    number == total
}

fn get_cache() -> [i32; 10]
{
    let mut cache = [0; 10];    // init. with 0s
    // cache[0] == 0
    for n in 1 ..= 9 {
        cache[n] = i32::pow(n as i32, n as u32);
    }
    // println!("# {:?}", cache);
    cache
}

fn main()
{
    let cache: [i32; 10] = get_cache();

    for n in 0 .. N {
        // if (n > 0) && (n % 1_000_000 == 0) {
            // println!("# {}", n);
        // }
        if is_munchausen(n, &cache) {
            println!("{}", n);
        }
    }
}
