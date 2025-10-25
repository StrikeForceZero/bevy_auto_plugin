use std::ops::{Deref, DerefMut};

pub struct AfterDrop<'a, T, F: FnOnce(&'a mut T)> {
    target: Option<&'a mut T>,
    on_drop: Option<F>,
}

impl<'a, T, F: FnOnce(&'a mut T)> AfterDrop<'a, T, F> {
    pub fn new(target: &'a mut T, on_drop: F) -> Self {
        Self {
            target: Some(target),
            on_drop: Some(on_drop),
        }
    }
}

impl<'a, T, F: FnOnce(&'a mut T)> Deref for AfterDrop<'a, T, F> {
    type Target = T;
    fn deref(&self) -> &T {
        self.target.as_deref().unwrap()
    }
}
impl<'a, T, F: FnOnce(&'a mut T)> DerefMut for AfterDrop<'a, T, F> {
    fn deref_mut(&mut self) -> &mut T {
        self.target.as_deref_mut().unwrap()
    }
}

impl<'a, T, F: FnOnce(&'a mut T)> Drop for AfterDrop<'a, T, F> {
    fn drop(&mut self) {
        if let (Some(t), Some(f)) = (self.target.take(), self.on_drop.take()) {
            f(t);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use internal_test_proc_macro::xtest;
    #[xtest]
    fn test_after_drop() {
        let mut x = 0;
        let mut y = AfterDrop::new(&mut x, |x| *x = 1);
        *y = 2;
        drop(y);
        assert_eq!(x, 1);
    }
}
