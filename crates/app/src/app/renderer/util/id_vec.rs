use std::{num::NonZeroU32, ops::Range};

/// A contiguous growable array type. It assigns to each element
/// reusable ids that do not change on any `IdVec` operations.
#[derive(Debug)]
pub struct IdVec<T> {
    elements: Vec<T>,
    /// Relation [element index] => id
    elements_ids: Vec<Id>,

    /// Relation [id] => element index
    /// Also:    [unused id] => other unused id
    ids_indexes: Vec<PackedIdEntry>,

    /// It is the most recently used vacant id.
    /// Using the most recently used should be more cache friendly.
    first_vacant_id: Option<Id>,

    /// Smallest range that contains all the updated elements
    /// from the last call of `reset_updated_range`.
    updated_range: Range<usize>,
}

type Id = u32;

#[derive(Debug)]
enum IdEntry {
    /// The Id is in use.
    Occupied { element_index: usize },
    /// The Id is free (not used).
    Vacant { next_vacant_id: Option<Id> },
}

/// It packs IdEntry in half of the bytes.
/// Least significant bit is 0: Occupied, 1: Vacant.
/// Vacant { next_vacant_id: None } is u64::MAX
#[derive(Copy, Clone)]
struct PackedIdEntry(u32);

impl std::fmt::Debug for PackedIdEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        IdEntry::from(*self).fmt(f)
    }
}

impl From<IdEntry> for PackedIdEntry {
    fn from(value: IdEntry) -> Self {
        match value {
            IdEntry::Occupied { element_index } => Self((element_index as u32) << 1),
            IdEntry::Vacant {
                next_vacant_id: None,
            } => Self(u32::MAX),
            IdEntry::Vacant {
                next_vacant_id: Some(id),
            } => Self((id << 1) | 1),
        }
    }
}

impl From<PackedIdEntry> for IdEntry {
    fn from(packed: PackedIdEntry) -> IdEntry {
        if (packed.0 & 1 == 0) {
            IdEntry::Occupied {
                element_index: packed.0 as usize >> 1,
            }
        } else if (packed.0 != u32::MAX) {
            IdEntry::Vacant {
                next_vacant_id: Some(packed.0 >> 1),
            }
        } else {
            IdEntry::Vacant {
                next_vacant_id: None,
            }
        }
    }
}

impl<T> IdVec<T> {
    pub fn new() -> Self {
        Self {
            ids_indexes: vec![],
            elements_ids: vec![],
            elements: vec![],
            first_vacant_id: None,
            updated_range: 0..0,
        }
    }

    pub fn as_slice(&self) -> &[T] {
        self.elements.as_slice()
    }

    /// Will use the first_vacant_id if posible. If not it will create a new one.
    fn use_id(&mut self, element_index: usize) -> Id {
        if let Some(id) = self.first_vacant_id {
            let vacant = self.ids_indexes[id as usize];
            self.ids_indexes[id as usize] = IdEntry::Occupied { element_index }.into();
            match vacant.into() {
                IdEntry::Occupied { .. } => unreachable!(),
                IdEntry::Vacant { next_vacant_id } => self.first_vacant_id = next_vacant_id,
            }
            id
        } else {
            let id = self.ids_indexes.len() as Id;
            self.ids_indexes
                .push(IdEntry::Occupied { element_index }.into());
            id
        }
    }

    pub fn push(&mut self, element: T) -> Id {
        let element_index = self.elements.len();
        let element_id = self.use_id(element_index);

        self.elements_ids.push(element_id);
        self.elements.push(element);
        self.set_updated(element_index);

        element_id
    }

    pub fn get(&self, element_id: Id) -> Option<&T> {
        match (*self.ids_indexes.get(element_id as usize)?).into() {
            IdEntry::Occupied { element_index } => Some(&self.elements[element_index]),
            IdEntry::Vacant { .. } => None,
        }
    }

    pub fn get_mut(&mut self, element_id: Id) -> Option<&mut T> {
        match (*self.ids_indexes.get(element_id as usize)?).into() {
            IdEntry::Occupied { element_index } => {
                self.set_updated(element_index);
                Some(&mut self.elements[element_index])
            }
            IdEntry::Vacant { .. } => None,
        }
    }

    /// Very efficient swap remove
    pub fn remove(&mut self, element_id: Id) {
        let Some(element) = self.ids_indexes.get(element_id as usize) else {
            return;
        };

        match (*element).into() {
            IdEntry::Occupied { element_index } => {
                let next_vacant_id = self.first_vacant_id;
                self.first_vacant_id = Some(element_id);

                // Delete
                self.elements.swap_remove(element_index);
                self.elements_ids.swap_remove(element_index);
                self.set_updated(element_index);
                self.ids_indexes[element_id as usize] = IdEntry::Vacant { next_vacant_id }.into();

                // Replace
                if let Some(moved_id) = self.elements_ids.get(element_index) {
                    self.ids_indexes[*moved_id as usize] =
                        IdEntry::Occupied { element_index }.into();
                }
            }
            IdEntry::Vacant { .. } => {}
        }
    }

    /// This function should be called every time `elements` is mutated.
    fn set_updated(&mut self, index: usize) {
        if index < self.len() {
            if self.updated_range.is_empty() {
                self.updated_range = index..index + 1;
            } else if index < self.updated_range.start {
                self.updated_range.start = index;
            } else if self.updated_range.end < index {
                self.updated_range.end = index;
            }
        }
    }

    pub fn reset_updated_range(&mut self) -> Range<usize> {
        let range = self.updated_range.clone();
        self.updated_range = 0..0;

        if self.len() <= range.start {
            0..0
        } else if range.end <= self.len() {
            range.start..self.len()
        } else {
            range
        }
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_into_idvec() {
        let mut v = IdVec::new();
        assert!(v.push('a') == 0);
        assert!(v.push('a') == 1);
        assert!(v.push('b') == 2);

        assert!(v.as_slice() == &['a', 'a', 'b']);

        v.remove(0);

        println!("{:?}", v);
        assert!(v.as_slice() == &['b', 'a']);
        assert!(v.get(0) == None);
        assert!(v.get(1) == Some(&'a'));
        assert!(v.get(2) == Some(&'b'));
        assert!(v.get(3) == None);

        assert!(v.push('c') == 0);

        assert!(v.as_slice() == &['b', 'a', 'c']);
        assert!(v.get_mut(0) == Some(&mut 'c'));
        assert!(v.get_mut(1) == Some(&mut 'a'));
        assert!(v.get_mut(2) == Some(&mut 'b'));
        assert!(v.get_mut(3) == None);

        assert!(v.len() == 3);

        v.remove(0);
        assert!(v.len() == 2);

        v.remove(0);
        assert!(v.len() == 2);

        v.remove(1);
        v.remove(2);
        assert!(v.len() == 0);
    }
}
