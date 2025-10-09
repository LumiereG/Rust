use std::io;
use rand::Rng;
use std::fs::File;
use std::io::Write;

fn main() {
    let last: bool = loop{
        println!("Podaj liczbę");
        let mut string = String::new();

        io::stdin()
            .read_line(&mut string)
            .expect("Failed to read line");

        let number: u64 = match string.trim().parse(){
            Ok(num) => num,
            Err(_) => break true,
        };

        if number == 0 {
            break false;
        }

        let number = rand::thread_rng().gen_range(0..=5) + number;
        println!("Nowa liczba {number}");

        let table = powers(number);
        println!("{:?}", table);

        let collatz_condition = check_table_collatz(table);
        println!("{:?}", collatz_condition);

        let mut file = match File::create("xyz.txt") {
            Ok(file) => file,
            Err(_) => {
                println!("Nie udało się utworzyć pliku.");
                break true;
            }
        };

        let content = format!("{:?}", collatz_condition);
        if let Err(e) = file.write_all(content.as_bytes()) {
            println!("Błąd zapisu do pliku: {}", e);
            break true;
        }
    };

    let max_iter = 5;
    let (sum, was_broken) = super_func(max_iter);
    println!("Suma wygenerowanych liczb: {}", sum);
    println!("Czy pętla została przerwana? {}", was_broken);

    if last{
        println!("Pętla się skończyła z powodu błedu");
    }
    else {
        println!("Pętla się skończyła z woli użytkownika");
    }
}

fn powers(x: u64) -> [u64; 10]
{
    let mut table: [u64; 10] = [x; 10];
    for ind in 1..10
    {
        table[ind] = x*table[ind-1];
    }
    table
}

fn check_table_collatz(numbers:[u64;10]) -> [bool;10]{
    let mut results: [bool; 10] = [false; 10];
    for i in 0..10 
    {
        results[i] = check_collatz(numbers[i]);
    }
    results
}

fn check_collatz(mut n: u64) -> bool {
    for _ in 0..100 {
        if n == 1 {
            return true;
        }
        if n.is_multiple_of(2){
            n /= 2;
        } else{
            n = 3*n + 1;
        }
    }
    false
}

fn super_func(n: i32) -> (i32, bool) {
    let mut total_sum = 0;
    let mut was_broken = false;
    'outer: for _ in 1..=n {
        for _ in 1..=10 {
            let random_value = rand::thread_rng().gen_range(1..=100);
            if random_value == 50 {
                was_broken = true;
                break 'outer; 
            }
            total_sum += random_value;
        }
    }
    (total_sum, was_broken)
}