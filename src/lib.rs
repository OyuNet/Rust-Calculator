pub fn summary(num1: i32, num2: i32) -> i32 {
    return num1+num2;
}

pub fn subtraction(num1: i32, num2: i32) -> i32 {
    return num1-num2;
}

pub fn multiplication(num1: i32, num2: i32) -> i32 {
    return num1*num2;
}

pub fn dividing(num1: i32, num2: i32) -> i32 {
    return num1/num2
}

pub fn exponent(base: i32, power: i32) -> i32 {
    let mut pwr = power - 1;
    let mut result = base;
    while pwr > 0 {
        result = result * base;
        pwr = pwr - 1;
    }

    return result;
}

pub fn root(num: f64, power: f64) -> f64 {
    return num.powf(1 as f64 / power);
}

pub fn sqrt(num: i32) -> f64 {
    return root(num as f64, 2 as f64);
}

pub fn cbrt(num: i32) -> f64 {
    return root(num as f64, 3 as f64);
}