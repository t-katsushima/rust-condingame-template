#![allow(dead_code)]
pub trait ExtVec<T: Clone> {
    fn head_option(&self) -> Option<T>;
}
impl<T: Clone> ExtVec<T> for Vec<T> {
    fn head_option(&self) -> Option<T> {
        if self.len() == 0 {
            None
        } else {
            Some(self[0].clone())
        }
    }
}
