
fn index(idx: usize, arr: &[u8], unsound: bool) -> Option<u8> {
    let bounds_check = match unsound {
        true => idx <= arr.len(), //  UB !
        false => idx < arr.len()
    };
    if bounds_check { 
        unsafe {
            Some(*arr.get_unchecked(idx))
        }
    } else {
        None
    }
}

use std::ptr;

pub struct NaiveVec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}

// Note: does not correctly handle zero-sized types.
impl<T> NaiveVec<T> {
    pub fn push(&mut self, elem: T) {
        if self.len == self.cap {
            // TODO:
            // self.reallocate();
        }
        unsafe {
            ptr::write(self.ptr.add(self.len), elem);
            self.len += 1;
        }
    }

    fn make_room(&mut self) {
        // grow the capacity
        self.cap += 1; // UB !!!
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn test_index(){
        let v: Vec<u8> = vec![1,2,3];
        let a: &[u8] = v.as_slice();
        let ui = unsafe {
            *a.get_unchecked(0)
        };
        assert_eq!(index(0,a, false), Some(1));
        assert_eq!(index(3,a, false), None);
        assert_eq!(index(0,a, true), Some(1));
        let x = index(3,a, true); // UB - does not panic !
        println!("unchecked at index 0: {0}, unwrap at index 3: {1}", ui, x.unwrap()); // cargo test -- --show-output
        assert_eq!(x, Some(0));
    }

    
    
}
