use std::io;
mod funcs;
mod user;

fn main() {
    println!("Задайте вектор чисел через пробел:");

    let mut nums: Vec<f32> = Vec::new();
    user::inp(&mut nums);

    loop {
        if nums.is_empty() {
            println!("Необходимо заполнить вектор");
            user::inp(&mut nums);
            continue;
        }

        println!("Введите команду: ");
        let mut temp: String = String::new();
        let temp = match io::stdin().read_line(&mut temp) {
            Ok(_) => temp.trim(),
            Err(_e) => continue,
        };

        let command: &str = temp;
        match command {
            "arith_mean" => println!("{}", funcs::arith_mean(&nums)),
            "geom_mean" => println!("{}", funcs::geom_mean(&nums)),
            "power_mean" => println!("{}", funcs::power_mean(&nums)),
            "arith_geom_mean" => println!("{}", funcs::arith_geom_mean(&nums)),
            "quasi_mean" => println!("{}", funcs::quasi_mean(&nums)),
            "mediana" => println!("{}", funcs::mediana(&nums)),
            "cut_arith_mean" => println!("{}", funcs::cut_arith_mean(&nums)),
            "vin_arith_mean" => println!("{}", funcs::vin_arith_mean(&nums)),
            "mode" => {
                let vec = stats::modes(nums.clone().into_iter()).0;
                println!("{}", vec.iter().sum::<f32>() / (vec.len() as f32))
            }
            "aver_square_dif" => println!("{}", funcs::disp(&nums).powf(0.5)),
            "disp" => println!("{}", funcs::disp(&nums)),
            "ald" => println!("{}", funcs::ald(&nums)),
            "liner_k_var" => println!("{}", funcs::ald(&nums) / funcs::arith_mean(&nums) * 100.0),
            "square_k_var" => println!("{}",funcs::disp(&nums).powf(0.5) / funcs::arith_mean(&nums) * 100.0),
            "delete" => user::delete(&mut nums),
            "put" => {
                println!("Задайте чисела через пробел");
                user::inp(&mut nums);
            }

            "vec" => println!(
                "{:?}",
                nums.clone()
                    .into_iter()
                    .map(|x| x as i32)
                    .collect::<Vec<i32>>()
            ),
            "end" => break,
            "help" => user::help(),

            _ => println!("Unknown command. try 'help' "),
        };
    }
}
