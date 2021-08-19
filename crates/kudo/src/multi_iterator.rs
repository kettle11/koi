pub struct MultiIterator<ITERATOR>(pub(crate) ITERATOR);

macro_rules! multi_iterator_impl {
    ( $count: tt, $( ($index: tt, $tuple:ident) ),* ) => {
        impl<$( $tuple: Iterator,)*> MultiIterator<($( $tuple,)*)> {
            pub fn new(iterators: ($( $tuple,)*)) -> Self {
                Self(iterators)
            }
        }

        impl<$( $tuple: Iterator,)*> Iterator for MultiIterator<($( $tuple,)*)> {
            type Item = ($( $tuple::Item,)*);
            fn next(&mut self) -> Option<Self::Item> {
                Some(($( self.0.$index.next()?,)*))
            }
        }
    }
}
