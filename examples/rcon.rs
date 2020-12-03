//! Simple REPL, which sends the inputted data to a minecraft server

use rcon::McRcon;

fn readline(prompt: &str) -> String {
    use std::io::Write;

    print!("{}", prompt);
    std::io::stdout().flush().expect("Failed to flush stdout");

    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Could not read line");
    line.trim_end().to_string()
}

fn main() {
    let host = readline("Hostname: ");
    let port: u16 = readline("Port: ").parse().expect("Invalid number");
    let password = readline("Password: ");
    println!();

    let mut rcon = McRcon::new((host, port), password).expect("Could not initialize rcon");

    loop {
        let command = readline("> ");
        if command.is_empty() {
            break;
        }

        println!(
            "{}",
            &rcon
                .command(command)
                .expect("Could not send command")
                .payload
        );
    }
}
