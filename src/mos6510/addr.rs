#[inline]
pub fn is_zero_page(num: i32) -> bool {
    num >= 0 && num <= 256
}
