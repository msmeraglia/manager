use std::ops::Range;

pub struct Manager<T> {
    data: Vec<T>,
    look_up: Vec<usize>,
    index: Vec<usize>,
    capacity: usize,
    free: usize,
    count: usize,
}
#[allow(dead_code)]
// Update to add version support, entity_id instead of usize
impl<T> Manager<T> {
    pub fn clear(&mut self) {
        self.count = 0;
        self.free = 0;
        self.data.clear();
        self.look_up.clear();
        self.index.clear();
        for i in 0..self.capacity {
            self.look_up.push(i + 1);
            self.index.push(0);
        }
    }

    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    pub fn get(&self, id: usize) -> Option<&T> {
        let index = self.index[id];
        self.data.get(index)
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut T> {
        let index = self.index[id];
        self.data.get_mut(index)
    }

    pub fn get_range(&self, ids: Range<usize>) -> &[T] {
        &self.data.as_slice()[ids]
    }

    pub fn get_range_mut(&mut self, ids: Range<usize>) -> &mut [T] {
        &mut self.data.as_mut_slice()[ids]
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn is_full(&self) -> bool {
        self.count >= self.capacity
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let mut look_up = Vec::with_capacity(capacity);
        let mut index = Vec::with_capacity(capacity);
        for i in 0..capacity {
            look_up.push(i + 1);
            index.push(0);
        }
        Self {
            data: Vec::with_capacity(capacity),
            look_up,
            index,
            capacity,
            count: 0,
            free: 0,
        }
    }
    pub fn push(&mut self, item: T) -> Option<usize> {
        if self.free > self.capacity {
            return None;
        }
        let index = self.free;

        self.free = self.look_up[index];
        self.look_up[index] = self.count;
        self.index[self.count] = index;
        self.data.push(item);
        self.count += 1;

        Some(index)
    }
    pub fn remove(&mut self, id: usize) {
        let slot_id = id;
        let data_id = self.index[slot_id];
        let lookup_to_fix = self.look_up[self.count - 1];
        self.index[slot_id] = self.free;
        self.index[lookup_to_fix] = data_id;
        self.data.swap_remove(data_id);
        self.look_up[data_id] = lookup_to_fix;
        self.free = slot_id;
        self.count -= 1;
    }
}
