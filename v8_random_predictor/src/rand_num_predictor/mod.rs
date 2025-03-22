use crate::rand_num_generator::Xorshift128;

pub fn predictor(nums_vec: Vec<f64>) -> Result<Xorshift128, String> {
    if nums_vec.len() < 3 {
        let vec_len: usize = nums_vec.len();
        return Err(format!(
            "The input should be of length >= 3, currently the length is {vec_len}"
        ));
    }

    fn get_state(mut num: f64) -> u64 {
        num = 1f64 + num;
        let num: u64 = (num * (1u64 << 53) as f64).round() as u64;
        num << 11
    }

    let state1: u64 = get_state(nums_vec[1]);
    let state2: u64 = get_state(nums_vec[2]);

    Ok(Xorshift128 { state1, state2 })
}
