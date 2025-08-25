#![allow(unused)]
#![allow(non_snake_case)]

unsafe extern "C" {
    fn malloc(size: usize) -> *mut u8;
    fn free  (ptr: *mut u8);
}

pub struct Ptr<T> {
    data: *mut T
}

impl<T> Ptr<T> {
    pub fn new(v: *mut T) -> Self {
	Self { data: v}
    }

    pub fn get(&self) -> &T {
	unsafe { &*self.data } 
    }

    pub fn get_and<F, R>(&mut self, f: F) -> R
    where F: FnOnce(&mut T) -> R{
	unsafe { f(&mut *self.data) }
    }

    pub fn raw(&self) -> *mut T {
	self.data
    }
    
    pub fn clone(&self) -> Ptr<T> {
	Ptr { data: self.data }
    }
	
}

pub struct Allocator {
    pointers: Vec<Ptr<u8>>,
}

impl Allocator {
    pub fn new() -> Self {
	Self { pointers: vec![] }
    }

    pub fn alloc(&mut self, size: usize) -> Ptr<u8> {
	unsafe {
	    let ptr = Ptr::new(malloc(size));
	    self.pointers.push(ptr.clone());
	    ptr
	}
    }

    pub fn get_ty<T>(&mut self) -> Ptr<T> {
	unsafe {
	    let mut ptr = Ptr::new(malloc(std::mem::size_of::<T>()));
	    self.pointers.push(ptr.clone());
	    Ptr::new(ptr.get_and(|x| x as *mut u8 as *mut T))
	}
    }
    
    pub fn freeAll(&mut self) {
	unsafe {
	    for ptr in &self.pointers {
		free(ptr.raw());
	    }
	}
    }
}
