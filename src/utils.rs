
pub fn is_prime(x: u64) -> bool {
    if x%2 == 0 {
        return false;
    } else {
        let sqrt_x: u64 = (x as f64).sqrt() as u64;
        for i in (3..sqrt_x).step_by(2) {
            if x%i == 0 {
                return false;
            }
        }
    }
    return true;
}
