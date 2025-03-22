use std::io::stdin;
use v8_random_predictor::rand_num_generator::Xorshift128;
use v8_random_predictor::rand_num_predictor::predictor;

fn main() {
    let mut state: Xorshift128 = Xorshift128::new(1233744567890, 3646874774744);

    let mut command: String = String::from("");
    println! {"Kindly choose the operation. (random_number => 1/\"\", predict_number => 2, quit => exit)"};

    loop {
        command.clear();
        match stdin().read_line(&mut command) {
            Ok(_o) => {}
            Err(e) => {
                println!("An unexpected error occured. {e}");
                command.push_str("exit")
            }
        };

        command = command.trim().to_string();

        match command.as_str() {
            "1" | "" => {
                let random_num: f64 = state.next();
                println!("{random_num}");
                continue;
            }
            "2" => {
                match predictor(vec![0.2089244119096274f64, 0.5543277367964399f64, 0.7575345605035386f64]) {
                    Ok(mut val) => {
                        let random_num: f64 = val.next();
                        println!("{random_num}");
                    }
                    Err(err) => {
                        println!("Unexpected error occured. {err}");
                        break;
                    }
                };
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
