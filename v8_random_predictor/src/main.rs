use std::io::stdin;
use v8_random_predictor::rand_num_generator::Xorshift128;

#[allow(unused)]
fn main() {
    let mut state: Xorshift128 = Xorshift128::new(1233744567890, 3646874774744);
    let command_tuple: (String, String) = (String::from("1"), String::from("2"));

    let mut command: String = String::from("");
    println! {"Kindly choose the operation. (random_number => 1/\"\", predict_number => 2, quit => exit)"};

    loop {
        command.clear();
        stdin().read_line(&mut command);

        command = command.trim().to_string();

        match command.as_str() {
            "1" | "" => {
                let random_num: f64 = state.next();
                println!("{random_num}");
                continue;
            }
            "2" => {
                println!("Yet to implement");
                continue;
            }
            "exit" => break,
            _ => {}
        }

        println!(
            "Kindly choose a vaild operation. Your chooice = [{command}]. (random_number => 1, predict_number => 2)"
        );

        command.clear();
    }
}
