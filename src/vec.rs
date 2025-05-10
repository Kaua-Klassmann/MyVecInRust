use std::{alloc, fmt::Debug, ops::Index, slice};

#[derive(Debug)]
pub struct MyVec<T> {
    length: usize,
    capacity: usize,
    layout: alloc::Layout,
    ptr: *mut T,
}

impl<T> MyVec<T> {
    pub fn new() -> MyVec<T> {
        let layout = alloc::Layout::array::<T>(1).unwrap();

        unsafe {
            let ptr = alloc::alloc(layout) as *mut T;

            if ptr.is_null() {
                panic!()
            }

            MyVec {
                length: 0,
                capacity: 1,
                layout,
                ptr,
            }
        }
    }

    pub fn with_capacity(capacity: usize) -> MyVec<T> {
        let layout = alloc::Layout::array::<T>(capacity).unwrap();

        unsafe {
            let ptr = alloc::alloc(layout) as *mut T;

            if ptr.is_null() {
                panic!()
            }

            MyVec {
                length: 0,
                capacity,
                layout,
                ptr,
            }
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn clear(&mut self) {
        let layout = alloc::Layout::array::<T>(1).unwrap();

        unsafe {
            let ptr = alloc::alloc(layout) as *mut T;

            if ptr.is_null() {
                panic!()
            }

            for index in 0..self.length {
                self.ptr.add(index).drop_in_place();
            }

            self.length = 0;
        }
    }

    pub fn push(&mut self, element: T) {
        if self.length == self.capacity {
            let new_layout = alloc::Layout::array::<T>(self.capacity * 2).unwrap();

            unsafe {
                let temp_ptr = alloc::alloc(new_layout) as *mut T;

                if temp_ptr.is_null() {
                    panic!();
                }

                for index in 0..self.capacity {
                    let val = self.ptr.add(index).read();
                    temp_ptr.add(index).write(val);
                }

                alloc::dealloc(self.ptr as *mut u8, self.layout);

                self.capacity *= 2;
                self.layout = new_layout;
                self.ptr = temp_ptr;
            }
        }

        unsafe {
            self.ptr.add(self.length).write(element);
            self.length += 1;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            if self.length == 0 {
                return None;
            }

            self.length -= 1;

            Some(self.ptr.add(self.length - 1).read())
        }
    }
}

impl<T, Idx: slice::SliceIndex<[T]>> Index<Idx> for MyVec<T> {
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        unsafe {
            let slice = slice::from_raw_parts(self.ptr, self.length);
            &slice[index]
        }
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        unsafe {
            for index in 0..self.length {
                self.ptr.add(index).drop_in_place();
            }

            alloc::dealloc(self.ptr as *mut u8, self.layout);
        }
    }
}

impl<T> Clone for MyVec<T> {
    fn clone(&self) -> Self {
        MyVec {
            length: self.length,
            capacity: self.capacity,
            layout: self.layout,
            ptr: self.ptr,
        }
    }
}
