use std::io;
// overload of f64???
pub fn inp(numbers: &mut Vec<f32>) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("ha ha");
    for word in input.split_whitespace() {
        match word.parse::<f32>() {
            Ok(num) => numbers.push(num),
            Err(_) => println!("{} NaN", word),
        }
    }
}

// input of 1 num
pub fn input_one() -> f32 {
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

pub fn delete(numbers: &mut Vec<f32>) {
    loop {
        let index: usize = match (input_one() as i32).try_into() {
            Ok(result) => result,
            Err(_) => {
                println!("Try something other");
                continue;
            }
        };
        if index <= numbers.len(){
            numbers.remove((index - 1) as usize);
            break;
        } else {
            println!("Попробуйте что-то другое");
            continue;
        }
    }
}

pub fn help() {
    println!("Если команда требует доп. значений, то она потребует их после ее ввода");
    println!("Доступные команды");
    println!("arith_mean        - среднее арифметическое");
    println!("geom_mean         - среднее геометрическое");
    println!("power_mean        - среднее степенное");
    println!("arith_geom_mean   - арифметико-геометрическое среднее(модфицированное)");
    println!("quasi_mean        - среднее Колмогорова ");
    println!("mediana           - медиана");
    println!("cut_arith_mean    - среднее усеченное");
    println!("vin_arith_mean    - винсоризованное среднее");
    println!("mode              - мода(в случае четности берется среднее из двух)");
    println!("aver_square_dif   - среднее квадратическое отклонение");
    println!("ald               - среднее линейное отклонение");
    println!("liner_k_var       - линейный коэффициент вариации");
    println!("square_k_var      - квадратический коэффициент вариации");
    println!("disp              - дисперсия");
    println!("delete            - удаление n-го элемента вектора");
    println!("put               - добавление в вектор элементов из строки(все 'не цифры пропускаются')");
    println!("vec               - показывает вектор на данный момент");
    println!("end               - заканчивает работу");
}
