use third::funcs;

//функции, отсутвующие тут, делаются прямо в main из других(прим.:liner_k_disp)
//для удобства и осуществления проверки пришлось пожертвовать читаемостью main
//надеюсь я не запутался в значениях

#[cfg(test)]
// let nums: Vec<f32>  = vec![1.0,4.0,5.0,6.0,6.0,7.0,7.0,7.0,7.0,7.0,20.0,132.0,202.0,0.0,234.0];
#[test]
fn is_arith_mean_correct() {
    let nums: Vec<f32> = vec![
        1.0, 4.0, 5.0, 6.0, 6.0, 7.0, 7.0, 7.0, 7.0, 7.0, 20.0, 132.0, 202.0, 0.0, 234.0,
    ];
    assert_eq!(funcs::arith_mean(&nums), 43.0);
}

#[test]
fn is_geom_mean_correct() {
    let nums: Vec<f32> = vec![
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 6.0, 6.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 7.0, 20.0, 132.0,
        202.0, 234.0,
    ];
    assert_eq!((funcs::geom_mean(&nums) * 100.0).round() / 100.0, 9.33)
}

#[test]
fn is_power_mean_correct() {
    let nums: Vec<f32> = vec![
        1.0, 4.0, 5.0, 6.0, 6.0, 7.0, 7.0, 7.0, 7.0, 7.0, 20.0, 132.0, 202.0, 0.0, 234.0,
    ];
    assert_eq!(funcs::power_mean(&nums, 5.0), 627183000000.0);
}

#[test]
fn is_arith_geom_mean_correct() {
    let nums: Vec<f32> = vec![
        1.0, 4.0, 5.0, 6.0, 6.0, 7.0, 7.0, 7.0, 7.0, 7.0, 20.0, 132.0, 202.0, 0.0, 234.0,
    ];
    assert_eq!(funcs::arith_geom_mean(&nums, 0.7), 5.085726e-5);
}

#[test]
fn is_cut_arith_mean_correct() {
    let nums: Vec<f32> = vec![
        1.0, 4.0, 5.0, 6.0, 6.0, 7.0, 7.0, 7.0, 7.0, 7.0, 20.0, 132.0, 202.0, 0.0, 234.0,
    ];
    assert_eq!(funcs::cut_arith_mean(&nums, 25), 26.733334);
}

#[test]
fn is_vin_arith_mean_correct() {
    let nums: Vec<f32> = vec![
        1.0, 4.0, 5.0, 6.0, 6.0, 7.0, 7.0, 7.0, 7.0, 7.0, 20.0, 132.0, 202.0, 0.0, 234.0,
    ];
    assert_eq!(funcs::vin_arith_mean(&nums, 25), 41.0);
}

#[test]
fn is_mediana_correct() {
    let nums: Vec<f32> = vec![
        1.0, 4.0, 5.0, 6.0, 6.0, 7.0, 7.0, 7.0, 7.0, 7.0, 20.0, 132.0, 202.0, 0.0, 234.0,
    ];
    assert_eq!(funcs::median(&nums), 7.0);
}

//честно, я пытался с чатом посчитать, но он слишком тупенький даже для ср. ариф.
#[test]
fn is_ald_correct() {
    let nums: Vec<f32> = vec![
        1.0, 4.0, 5.0, 6.0, 6.0, 7.0, 7.0, 7.0, 7.0, 7.0, 20.0, 132.0, 202.0, 0.0, 234.0,
    ];
    assert_eq!(funcs::ald(&nums), 58.533333);
}

#[test]
fn is_disp_correct() {
    let nums: Vec<f32> = vec![
        1.0, 4.0, 5.0, 6.0, 6.0, 7.0, 7.0, 7.0, 7.0, 7.0, 20.0, 132.0, 202.0, 0.0, 234.0,
    ];
    assert_eq!(funcs::disp(&nums), 5733.8667);
}

#[test]
fn is_quasi_mean_correct() {
    let nums: Vec<f32> = vec![
        1.0, 4.0, 5.0, 6.0, 6.0, 7.0, 7.0, 7.0, 7.0, 7.0, 20.0, 132.0, 202.0, 0.0, 234.0,
    ];
    assert_eq!(funcs::arith_geom_mean(&nums, 1.0), 8.201599e-5);
}
