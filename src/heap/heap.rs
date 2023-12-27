use std::fmt::Debug;

#[allow(unused_variables)]
#[derive(Debug)]
pub struct MinHeap<T> {
    pub data: Vec<T>,
}

impl<T> MinHeap<T>
where
    T: Ord + Copy + Debug + PartialEq,
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

    pub fn top(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let root = self.data.swap_remove(0);

        self.heapify(0);

        Some(root)
    }

    /// Binary Tree to MinHeap
    pub fn heapify(&mut self, index: usize) {
        // [22, 25, 12, 10, 23]
        //   0  1   2   3   4
        //   Binary Tree
        //           22
        //          /  \
        //         25  12
        //        / \
        //      10  23
        //
        //   Min-Heap would be:
        //           22
        //          /  \
        //         10  12 swap(25, 10)
        //        / \
        //      25  23
        //  ---
        //           10
        //          /  \
        //         22  12 swap(22, 10)
        //        / \
        //      25  23
        //  ---
        let mut current_idx = index;
        let len_heap = self.data.len();

        let left_child_idx = self.get_left_child(index);
        let right_child_idx = self.get_right_child(index);

        if left_child_idx < len_heap && self.data[left_child_idx] < self.data[current_idx] {
            current_idx = left_child_idx;
        }

        if right_child_idx < len_heap && self.data[right_child_idx] < self.data[current_idx] {
            current_idx = right_child_idx;
        }

        if current_idx != index {
            self.data.swap(index, current_idx);
            self.heapify(current_idx);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    /// Get the root element in Heap and don't remove it.
    pub fn peek(&self) -> Option<&T> {
        self.data.get(0)
    }
}

mod test {
    use super::*;

    #[test]
    fn it_works() {
        let mut heap = MinHeap::<i32>::new();
        heap.insert(12);
        heap.insert(22);
        heap.insert(2);
        heap.insert(-1);
        heap.insert(-3);

        assert_eq!(heap.data, vec![-3, -1, 12, 22, 2]);
    }

    #[test]
    fn it_gets_top() {
        let mut heap = MinHeap::<i32>::new();
        heap.insert(12);
        heap.insert(22);
        heap.insert(2);
        heap.insert(-1);
        heap.insert(-3);

        let top = heap.top();
        assert_eq!(top, Some(-3));
    }

    #[test]
    fn it_top_and_heapifies() {
        let mut heap = MinHeap::<i32>::new();
        heap.insert(12);
        heap.insert(22);
        heap.insert(2);
        heap.insert(-1);
        heap.insert(-3);

        let top = heap.top();
        assert_eq!(top, Some(-3));
        assert_eq!(heap.data, vec![-1, 2, 12, 22]);
    }

    #[test]
    fn it_gets_root_and_stays() {
        let mut heap = MinHeap::<i32>::new();
        heap.insert(12);
        heap.insert(22);
        heap.insert(2);
        heap.insert(-1);
        heap.insert(-3);

        let peek = heap.peek();
        assert_eq!(peek, Some(-3).as_ref());
        assert_eq!(heap.data, vec![-3, -1, 12, 22, 2]);
    }
}
