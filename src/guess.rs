fn guess() {
    println!("guess the number!");

    let secret_number = rand::thread_rng()
        .gen_range(1, 101);

    println!("the secret number is {}", secret_number);

    loop {
        println!("please input your guess.");

        let mut guess = string::new();

        io::stdin().read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            ok(num) => num,
            err(_) => continue,
        };

        print!("you guessed: {}", guess);

        match guess.cmp(&secret_number) {
            ordering::less => println!(" to small!"),
            ordering::greater => println!(" too big!"),
            ordering::equal => {
                println!(" you win!");
                break;
            },
        }
    }
}