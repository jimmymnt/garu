#[allow(unused_variables)]
#[derive(Debug)]
pub struct MinHeap<T> {
    pub data: Vec<T>,
}

impl<T> MinHeap<T>
where
    T: Copy + Ord,
{
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn insert(&mut self, element: T) {
        self.data.push(element);
        let mut index = self.data.len() - 1;

        while index != 0 {
            let parent_idx = self.get_parent(index);

            if self.data[index] < self.data[parent_idx] {
                self.data.swap(parent_idx, index);
            }

            index = parent_idx;
        }
    }

    pub fn remove_by_index(&mut self, index: usize) {}

    pub fn get_parent(&self, index: usize) -> usize {
        (index - 1) / 2
    }

    pub fn get_left_child(&self, index: usize) -> usize {
        2 * index + 1
    }

    pub fn get_right_child(&self, index: usize) -> usize {
        2 * index + 2
    }
}
