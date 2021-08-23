pub struct OptionIterator<T: Iterator> {
    iter: Option<T>,
}

impl<T: Iterator> OptionIterator<T> {
    pub fn new(iter: Option<T>) -> Self {
        Self { iter }
    }
}

impl<T: Iterator> Iterator for OptionIterator<T> {
    type Item = Option<T::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(iter) = &mut self.iter {
            Some(iter.next())
        } else {
            Some(None)
        }
    }
}
