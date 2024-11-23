use std::vec;

pub fn arith_mean(nums: &[f32]) -> f32 {
    nums.iter().sum::<f32>() / nums.len() as f32
}

pub fn geom_mean(nums: &[f32]) -> f32 {
    nums.iter()
        .fold(1.0, |acc, &x| acc * x)
        .powf(1.0 / (nums.len() as f32))
}

// if can be used insted 2 previous func, but not comletely
pub fn power_mean(nums: &[f32], p: f32) -> f32 {
    nums.iter().fold(0.0, |acc, &x| acc + x.powf(p)) / (nums.len() as f32).powf(1.0 / p)
}

pub fn arith_geom_mean(nums: &[f32], w: f32) -> f32 {
    let mut ari_mean = arith_mean(nums);
    let mut geo_mean = geom_mean(nums);
    //лучше черз mut или через let в цикле
    loop {
        if (ari_mean - geo_mean).abs() < 0.0001 {
            return ari_mean;
        }
        let previous = vec![ari_mean * w, geo_mean * (1.0 - w)];
        ari_mean = arith_mean(&previous);
        geo_mean = geom_mean(&previous);
    }
}

// need to check if nums not empty in input
pub fn cut_arith_mean(nums: &[f32], perc: i32) -> f32 {
    let cut = ((nums.len() as i32) * perc / 100) as usize;
    if cut == 0 && perc != 0 {
        nums[1..=nums.len() - 1].iter().sum::<f32>() / nums.len() as f32
    } else {
        nums[cut..=nums.len() - cut].iter().sum::<f32>() / nums.len() as f32
    }
}

// что будет при perc = 0
pub fn vin_arith_mean(nums: &[f32], perc: i32) -> f32 {
    let mut cut = ((nums.len() as i32) * perc / 100) as usize;
    if cut == 0 && perc != 0 {
        cut = 1;
    }
    let mut sorted_num: Vec<_> = nums.to_vec();
    // а может двумя сортировками?(зато читаемо станет...)
    // need to check this part
    sorted_num.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let ext = [sorted_num[cut], sorted_num[sorted_num.len() - cut]].to_vec();
    // увеличиваем мин и макс оставшиеся цифры в количестве
    let ext: Vec<f32> = ext.iter().cloned().cycle().take(ext.len() * cut).collect();
    let ext = ext.as_slice();
    // выкидываем не нужное добавляем цифры
    [&sorted_num[cut..=nums.len() - cut], ext]
        .concat()
        .iter()
        .sum::<f32>()
        / sorted_num.len() as f32
}

pub fn median(nums: &[f32]) -> f32 {
    let mut sorted_num: Vec<_> = nums.to_vec();
    sorted_num.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if sorted_num.len() % 2 != 0 {
        sorted_num[sorted_num.len() / 2 + 1]
    } else {
        (sorted_num[sorted_num.len() / 2] + sorted_num[sorted_num.len() / 2 + 1]) / 2.0
    }
}

// было бы классно в классы запихнуть с наследованием, но я не знаю ооп
pub fn ald(nums: &[f32]) -> f32 {
    let mean = arith_mean(nums);
    (nums.iter().map(|x| (x - mean).abs()).sum::<f32>()) / nums.len() as f32
}

pub fn disp(nums: &[f32]) -> f32 {
    let mean = arith_mean(nums);
    (nums.iter().map(|x| (x - mean).powf(2.0)).sum::<f32>()) / nums.len() as f32
}

fn funcs(x: f32) -> f32 {
    2.7_f32.powf(x) + 2.0 * (x.powf(2.0)) + x + 5.0
}

// я не понял как работает метод бисекции
// пишу колму перед коллоком по матану
fn inverse_funcs(y: f32) -> f32 {
    let c: f32 = 58.028;
    if y > c {
        ((y - 58.028) / 1.020).ln()
    } else {
        ((y / 2.0) / 1.020).ln()
    }
}

pub fn quasi_mean(num: &Vec<f32>, _func_num: i32) -> f32 {
    //по идее здесь должен быть выбор функции но сделана только одна

    let big_sum: Vec<f32> = num.to_owned().clone().into_iter().map(funcs).collect();
    inverse_funcs(big_sum.iter().sum::<f32>() / big_sum.len() as f32)
}
