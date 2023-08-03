pub fn is_armstrong_number(_num: u32) -> bool {
    let count = _num.to_string().len() as u32;
    let mut sum = 0u32;
    let mut num = _num;

    while num > 0 {
        let overflowing_num = sum.overflowing_add((num % 10).pow(count));
        if overflowing_num.1 {
            return false;
        }
        sum = overflowing_num.0;

        num /= 10;
    }

    dbg!(sum, _num);

    sum == _num
}
