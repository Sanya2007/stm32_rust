use core::intrinsics::volatile_store;
use core::intrinsics::volatile_load;

pub struct VolatileReg32 {
    value: *mut u32
}

impl VolatileReg32 {
    pub fn new(value: *mut u32) -> VolatileReg32 {
        VolatileReg32 {
            value: value,
        }
    }

    pub fn new_offset(value: *mut u32, offset: u32) -> VolatileReg32 {
        VolatileReg32 {
            value: unsafe { value.offset(offset as isize) },
        }
    }

    pub fn get(&self) -> u32 {
        unsafe {
            volatile_load(self.value)
        }
    }

    pub fn set(&self, value: u32) {
        unsafe {
            volatile_store(self.value, value);
        }
    }

    pub fn bit_or(&self, value: u32) {
        let mut temp: u32 = self.get();
        temp |= value;
        self.set(temp);
    }

    pub fn bit_and(&self, value: u32) {
        let mut temp: u32 = self.get();
        temp &= value;
        self.set(temp);
    }

    pub fn bit_xor(&self, value: u32) {
        let mut temp: u32 = self.get();
        temp ^= value;
        self.set(temp);
    }

    pub fn add(&self, value: u32) {
        let mut temp: u32 = self.get();
        temp += value;
        self.set(temp);
    }

    pub fn sub(&self, value: u32) {
        let mut temp: u32 = self.get();
        temp -= value;
        self.set(temp);
    }
}

