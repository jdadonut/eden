use std::{ptr::null};


/// JArray is a fast implementation of a Java array with a fixed size.
/// Retrueving elements is an O(1) operation, while adding elements is possibly an O(n) operation.
/// We reserve a layout of memory sized (*mut T) * size, and then we can index into it and dereference
/// the pointer to get the value.
/// 
/// # Safety
/// The safety of this struct can be guaranteed to a reasonable degree- however, it is not guaranteed
/// in every case. There will always be a way to break continuity, no matter how many steps have been 
/// taken to ensure that this is not possible.
/// 
pub struct JArray<T, const SIZE: usize> {
    data: [*mut T; SIZE],
}
impl<T, const SIZE: usize> JArray<T, SIZE> {
    pub fn new_uninit() -> Self {
        Self {
            data: [null::<T>() as *mut T; SIZE],
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= SIZE {
            return None;
        }
        if self.data[index].is_null() {
            return None;
        }
        unsafe {
            Some(&*self.data[index])
        }
    }
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= SIZE {
            return None;
        }
        if self.data[index].is_null() {
            return None;
        }
        unsafe {
            Some(&mut *self.data[index])
        }
    }
    pub fn set(&mut self, index: usize, value: T) -> Option<()> {
        if index >= SIZE {
            return None;
        }
        if self.data[index].is_null() {
            return None;
        }
        unsafe {
            let old_ptr = self.data[index];
            *self.data[index] = value;
            if !old_ptr.is_null() {
                std::mem::drop(old_ptr);
            }
        }
        Some(())
    }

}

