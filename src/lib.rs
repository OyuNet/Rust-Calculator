pub fn summary(num1: i32, num2: i32) -> i32 {
    return num1+num2;
}

pub fn subtraction(num1: i32, num2: i32) -> i32 {
    return num1-num2;
}

pub fn multiplication(num1: i32, num2: i32) -> i32 {
    return num1*num2;
}

pub fn dividing(num1: f64, num2: f64) -> f64 {
    return num1/num2;
}

pub fn exponent(base: f64, power: f64) -> f64 {
    let mut pwr = power - (1 as f64);
    let mut result = base;
    while pwr > 0 as f64 {
        result = result * base;
        pwr = pwr - (1 as f64);
    }

    return result;
}

pub fn root(num: f64, power: f64) -> f64 {
    return num.powf(1 as f64 / power);
}

pub fn sqrt(num: f64) -> f64 {
    return root(num, 2 as f64);
}

pub fn cbrt(num: f64) -> f64 {
    return root(num as f64, 3 as f64);
}

pub fn sinx(adjacent: f64, oppose: f64) -> f64 {
    return oppose/sqrt(exponent(adjacent, 2 as f64) + exponent(oppose, 2 as f64));
}

pub fn cosx(adjacent: f64, oppose: f64) -> f64 {
    return adjacent/sqrt(exponent(adjacent, 2 as f64) + exponent(oppose, 2 as f64));
}

pub fn tanx(adjacent: f64, oppose: f64) -> f64 {
    return oppose/adjacent;
}

pub fn cotx(adjacent: f64, oppose: f64) -> f64 {
    return adjacent/oppose;
}

pub fn secx(adjacent: f64, oppose: f64) -> f64 {
    return (1 as f64)/cosx(adjacent, oppose);
}

pub fn cscx(adjacent: f64, oppose: f64) -> f64 {
    return (1 as f64)/sinx(adjacent, oppose);
}