use core::ptr::slice_from_raw_parts_mut;

pub struct PhysicalMemoryManager {
    bitmap: *mut [u8],
    used: usize,
}
impl PhysicalMemoryManager {
    fn bitmap(&self) -> &mut [u8] {
        unsafe { &mut *self.bitmap }
    }
    pub fn size(&self) -> usize {
        unsafe { &*self.bitmap }.len()
    }
    pub fn used(&self) -> usize {
        self.used
    }
    pub fn free(&mut self, page: usize) {
        self.bitmap()[page / 8] &= !(1 << (page % 8));
        self.used -= 1;
    }
    pub fn alloc(&mut self) -> Option<usize> {
        for i in 0..self.size() {
            if self.bitmap()[i / 8] & (1 << (i % 8)) == 0 {
                self.bitmap()[i / 8] |= 1 << (i % 8);
                self.used += 1;
                return Some(i);
            }
        }
        None
    }
    pub fn is_free(&self, page: usize) -> bool {
        self.bitmap()[page / 8] & (1 << (page % 8)) == 0
    }
    pub unsafe fn new(bitmap: *mut u8, size: usize, used: usize) -> Self {
        PhysicalMemoryManager {
            bitmap: slice_from_raw_parts_mut(bitmap, size),
            used,
        }
    }
}
