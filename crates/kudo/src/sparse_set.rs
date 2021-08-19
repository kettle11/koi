#[derive(Clone, Debug)]
pub struct SparseSet<T> {
    indices: Vec<Option<usize>>,
    data: Vec<T>,
    data_index_to_item_index: Vec<usize>,
}

impl<T> SparseSet<T> {
    pub fn new() -> Self {
        Self {
            indices: Vec::new(),
            data: Vec::new(),
            data_index_to_item_index: Vec::new(),
        }
    }

    pub fn insert(&mut self, index: usize, data: T) {
        // This line highlights the weakness of sparse sets.
        // They use a bunch of memory!
        if self.indices.len() <= index {
            self.indices.resize(index + 1, None);
        }

        let new_index = self.data.len();
        self.indices[index] = Some(new_index);

        // Is this a memory leak if an index is reused?

        self.data.push(data);

        self.data_index_to_item_index.push(index);
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        Some(&self.data[(*self.indices.get(index)?)?])
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    pub fn data_index_to_item_index(&self) -> &Vec<usize> {
        &self.data_index_to_item_index
    }
}
