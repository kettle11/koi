#[doc(hidden)]
/// A series of iterators of the same type that are traversed in a row.
pub struct ChainedIterator<I: Iterator> {
    current_iter: Option<I>,
    iterators: Vec<I>,
}

impl<I: Iterator> ChainedIterator<I> {
    #[doc(hidden)]
    pub fn new(mut iterators: Vec<I>) -> Self {
        let current_iter = iterators.pop();
        Self {
            current_iter,
            iterators,
        }
    }
}

impl<I: Iterator> Iterator for ChainedIterator<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        // Chain the iterators together.
        // If the end of one iterator is reached go to the next.
        loop {
            if let Some(iter) = &mut self.current_iter {
                if let v @ Some(_) = iter.next() {
                    return v;
                }
            }
            if let Some(i) = self.iterators.pop() {
                self.current_iter = Some(i);
            } else {
                return None;
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let mut min = 0;
        let mut max = 0;

        if let Some(current_iter) = &self.current_iter {
            let (i_min, i_max) = current_iter.size_hint();
            min += i_min;
            max += i_max.unwrap();
        }

        for i in self.iterators.iter() {
            let (i_min, i_max) = i.size_hint();
            min += i_min;
            // This function is designed under the assumption that all
            // iterators passed in implement size_hint, which works fine
            // for kudo's purposes.
            max += i_max.unwrap();
        }
        (min, Some(max))
    }
}

impl<I: ExactSizeIterator> ExactSizeIterator for ChainedIterator<I> {
    fn len(&self) -> usize {
        self.iterators.iter().fold(0, |count, i| count + i.len())
    }
}
