
use rand::Rng;

pub fn dice(sides: i32, number: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let mut result: i32 = 0;

    for i in 0..number {
        result = result + rng.gen_range(1..sides + 1)
    }

    result
}

pub fn dice_role(sides: i32, number: i32, minimum: i32) -> bool {
    dice(sides, number) >= minimum
}

