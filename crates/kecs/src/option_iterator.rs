pub struct OptionIterator<T: Iterator> {
    len_remaining: usize,
    iter: Option<T>,
}

impl<T: Iterator> OptionIterator<T> {
    pub fn new(len_remaining: usize, iter: Option<T>) -> Self {
        Self {
            len_remaining,
            iter,
        }
    }
}

impl<T: Iterator> Iterator for OptionIterator<T> {
    type Item = Option<T::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len_remaining == 0 {
            return None;
        }

        self.len_remaining -= 1;
        if let Some(iter) = &mut self.iter {
            Some(iter.next())
        } else {
            Some(None)
        }
    }
}
