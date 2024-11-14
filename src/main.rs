use std::io;
mod funcs;
mod user;


fn input_one() -> f32 {
    loop {
        let mut input = String::new();
        let index: f32 = match io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim().parse::<f32>() {
                Ok(num) => num,
                Err(_) => continue,
            },
            Err(_) => continue,
        };
        return index;
    }
}


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
            "power_mean" =>{ 
                let mut p;
                loop {
                    println!("Введите натуральную степень: ");
                    p = input_one();
                    if p % 1.0 == 0.0 && p > 0.0 {
                        break;
                    }
                }
                println!("{}", funcs::power_mean(&nums, p));
            },
            "arith_geom_mean" =>{
                let mut w;
                loop {
                    println!("Введите параметр веса от 0.0 до 1.0: ");
                    w = input_one();
                    if (0.0..=1.0).contains(&w) {
                        break;
                    }
                }
                println!("{}", funcs::arith_geom_mean(&nums, w))
            },
            "quasi_mean" =>{ 
                let mut func_num: i32;
                loop {
                    println!("Введите номер функции(1-1)");
                    func_num = input_one().floor() as i32;
                    if func_num != 1 {
                        println!("Только 1");
                        continue;
                    } else {
                        break;
                    }
                }
                println!("{}", funcs::quasi_mean(&nums, func_num))

            },
            "mediana" => println!("{}", funcs::mediana(&nums)),
            "cut_arith_mean" =>{ 
                let mut perc;
                loop {
                    println!("Введите процент(1-100) для удаления");
                    perc = input_one() as i32;
                    if 0 < perc && perc <= 100 {
                        break;
                    }
                }
                println!("{}", funcs::cut_arith_mean(&nums, perc))
            },
            "vin_arith_mean" =>{
                let mut perc;
                loop {
                    println!("Введите процент(1-100) для удаления");
                    perc = input_one() as i32;
                    if 0 < perc && perc <= 100 {
                        break;
                    }
                }
                println!("{}", funcs::vin_arith_mean(&nums, perc))
            },
            "mode" => {
                //реализация из stats
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

            _ => println!("Неизвестная команда. Попробуйте '`help'"),
        };
    }
}
