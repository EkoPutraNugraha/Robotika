use std::io;

fn main() {
    let mut position: (i32, i32) = (0, 0);

    loop {
        println!("Current position: ({}, {})", position.0, position.1);
        println!("Enter movement (w/a/s/d) or 'q' to quit:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();

        match input {
            "w" => position.1 = position.1.saturating_sub(1),
            "a" => position.0 = position.0.saturating_sub(1),
            "s" => position.1 += 1,
            "d" => position.0 += 1,
            "q" => {
                println!("Exiting. Final position: ({}, {})", position.0, position.1);
                break;
            }
            _ => println!("Invalid input. Use 'w', 'a', 's', 'd', or 'q'"),
        }
    }
}
