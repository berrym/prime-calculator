pub mod calculate {
    use colored::Colorize;
    use std::io;

    /// Use trial division to mark non prime indices in a boolean array false.
    fn mark_false(limit: usize, sieve: &mut Vec<bool>, progress: bool) -> io::Result<()> {
        sieve[0] = false;
        sieve[1] = false;

        for n in 2..=limit {
            if sieve[n] {
                let mut x = n * n;
                if progress {
                    println!("{} is prime", format!("{n}").green());
                }
                while x <= limit {
                    if progress {
                        println!("{} + {} = {}", format!("{n}").green(), format!("{x}").red(), n + x);
                    }
                    sieve[x] = false;
                    x += n;
                }
            }
        }
        Ok(())
    }

    /// Calculate primes up to limit using trial division
    pub fn n_primes(limit: usize, progress: bool) -> Vec<usize> {
        if limit < 2 {
            eprintln!("\n0 and 1 are undefined");
            return vec![];
        }

       // Create a mutable boolean vector to represent the sieve
        let mut primes = vec![true; limit + 1];

        // Mark all non prime numbers as false
        mark_false(limit, &mut primes, progress).unwrap();

        // Enumerate and filter_map primes for true values and return them
        primes
            .iter()
            .enumerate()
            .filter_map(|(n, &is_prime)| if is_prime { Some(n) } else { None })
            .collect()
    }

    /// Calculate the nth prime using the Sieve of Eratosthenes
    pub fn nth_prime(nth: usize, progress: bool) -> Option<usize> {
        if nth == 0 {
            return None;
        }

        let mut primes = vec![2];
        let mut counter = 3;

        if progress {
            println!("{} is prime", "2".green());
        }

        while primes.len() < nth {
            if primes
                .iter()
                .filter(|&x| counter % x == 0)
                .collect::<Vec<_>>()
                .is_empty()
            {
                if progress {
                    println!("{} is prime", format!("{counter}").green());
                }
                primes.push(counter);
            }
            if progress {
                let next = counter + 1;
                println!("{} is not prime", format!("{next}").red());
            }
            counter += 2
        }

        if let Some(n) = primes.last() {
            Some(*n)
        } else {
            None
        }
    }
}
