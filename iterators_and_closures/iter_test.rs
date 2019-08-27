fn main() {
    let nums = [1,2,3,4,5,6];

    let sum_of_evens = nums.iter().filter(|n| *n % 2 == 0).sum();
    let half_of_sum_of_evens = nums.iter().filter(|n| *n % 2 == 0).map(|n| n / 2).sum();
    assert_eq!(2 + 4 + 6, sum_of_evens);
    assert_eq!(sum_of_evens / 2, half_of_sum_of_evens);
}
