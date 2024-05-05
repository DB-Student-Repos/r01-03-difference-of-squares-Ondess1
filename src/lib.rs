pub fn square_of_sum(n: u32) -> u32 {
    let sum = (1..=n).sum::<u32>(); // Sum of the first N natural numbers
    sum * sum // Square of the sum
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..=n).map(|x| x * x).sum::<u32>() // Sum of the squares of the first N natural numbers
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n) // Difference between the square of the sum and the sum of the squares
}
