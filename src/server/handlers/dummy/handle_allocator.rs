use std::collections::HashMap;

pub struct HandleAllocator<T> {
    last_handle: usize,
    data: HashMap<usize, T>,
}

impl<T> HandleAllocator<T> {
    pub fn new(start_handle: usize) -> Self {
        Self {
            last_handle: start_handle,
            data: HashMap::new(),
        }
    }

    pub fn register(&mut self, value: T) -> usize {
        let new_handle = self.next_handle();
        self.data.insert(new_handle, value);

        new_handle
    }

    pub fn get(&self, handle: usize) -> Option<&T> {
        self.data.get(&handle)
    }

    pub fn get_mut(&mut self, handle: usize) -> Option<&mut T> {
        self.data.get_mut(&handle)
    }

    pub fn close(&mut self, handle: usize) -> Option<T> {
        self.data.remove(&handle)
    }

    fn next_handle(&mut self) -> usize {
        self.last_handle += 1;
        self.last_handle
    }
}
