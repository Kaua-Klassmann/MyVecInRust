use std::{alloc, marker::PhantomData, ops::Index, slice};

pub struct MyVec<T> {
    length: usize,
    capacity: usize,
    ptr: *mut T,
    _phantom: PhantomData<T>,
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
                capacity: 0,
                ptr,
                _phantom: PhantomData,
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
                ptr,
                _phantom: PhantomData,
            }
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn clear(&mut self) {
        unsafe {
            for index in 0..self.length {
                self.ptr.add(index).drop_in_place();
            }
        }

        self.length = 0;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if !(index < self.length) {
            return None;
        }

        unsafe { Some(&*self.ptr.add(index)) }
    }

    pub fn push(&mut self, element: T) {
        self.try_resize();

        unsafe {
            self.ptr.add(self.length).write(element);
            self.length += 1;
        }
    }

    pub fn insert(&mut self, index: usize, element: T) {
        self.try_resize();

        unsafe {
            for i in (index..self.length).rev() {
                let value = self.ptr.add(i).read();
                self.ptr.add(i + 1).write(value);
            }

            self.ptr.add(index).write(element);
            self.length += 1;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.length == 0 {
            return None;
        }

        self.length -= 1;

        unsafe { Some(self.ptr.add(self.length).read()) }
    }

    fn try_resize(&mut self) {
        if self.length == self.capacity {
            let old_layout = alloc::Layout::array::<T>(self.capacity).unwrap();
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

                alloc::dealloc(self.ptr as *mut u8, old_layout);
                self.ptr = temp_ptr;
            }

            self.capacity *= 2;
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
            let layout = alloc::Layout::array::<T>(self.capacity).unwrap();

            for index in 0..self.length {
                self.ptr.add(index).drop_in_place();
            }

            alloc::dealloc(self.ptr as *mut u8, layout);
        }
    }
}

impl<T: Clone> Clone for MyVec<T> {
    fn clone(&self) -> Self {
        let layout = alloc::Layout::array::<T>(self.capacity).unwrap();

        unsafe {
            let new_ptr = alloc::alloc(layout) as *mut T;

            if new_ptr.is_null() {
                panic!();
            }

            for index in 0..self.length {
                let value = (*self.ptr.add(index)).clone();
                new_ptr.add(index).write(value);
            }

            MyVec {
                length: self.length,
                capacity: self.capacity,
                ptr: new_ptr,
                _phantom: PhantomData,
            }
        }
    }
}
