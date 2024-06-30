/// It's similar to a HashMap<usize, T> but it assigns ids automatically.
///
/// # Performance benefits
///  - There's no hashing function
///  - There aren't collisions
///
/// # Warning
/// Ids are reused when removed as they are only indexes.
#[derive(Debug)]
pub struct IdVec<T> {
    data: Vec<T>,
    empty_slots: Vec<u32>,
}

impl<T> Default for IdVec<T> {
    fn default() -> Self {
        Self {
            data: Vec::default(),
            empty_slots: Vec::default(),
        }
    }
}

impl<T> IdVec<T> {
    pub fn add(&mut self, item: T) -> u32 {
        if let Some(index) = self.empty_slots.pop() {
            self.data[index as usize] = item;
            index
        } else {
            self.data.push(item);
            (self.data.len() - 1) as u32
        }
    }

    pub fn remove(&mut self, index: u32) {
        let last_index = self.data.len() - 1;
        if index == last_index as u32 {
            self.data.pop();
        } else {
            self.empty_slots.push(index);
        }
    }

    /// The slice will contain alive and death elements
    pub fn as_slice(&self) -> &[T] {
        self.data.as_slice()
    }
}
