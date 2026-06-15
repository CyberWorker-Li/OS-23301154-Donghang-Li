use alloc::vec::Vec;
use lazy_static::*;

use crate::config::{KERNEL_STACK_SIZE, PAGE_SIZE, TRAMPOLINE};
use crate::mm::{KERNEL_SPACE, MapPermission, VirtAddr, VirtPageNum};
use crate::sync::UPSafeCell;

pub struct PidHandle(pub usize);

struct PidAllocator {
    current: usize,
    recycled: Vec<usize>,
}

impl PidAllocator {
    pub fn new() -> Self {
        Self {
            current: 0,
            recycled: Vec::new(),
        }
    }
    pub fn alloc(&mut self) -> PidHandle {
        if let Some(pid) = self.recycled.pop() {
            PidHandle(pid)
        } else {
            self.current += 1;
            PidHandle(self.current - 1)
        }
    }
    pub fn dealloc(&mut self, pid: usize) {
        assert!(pid < self.current);
        assert!(self.recycled.iter().all(|&p| p != pid));
        self.recycled.push(pid);
    }
}

lazy_static! {
    static ref PID_ALLOCATOR: UPSafeCell<PidAllocator> = unsafe {
        UPSafeCell::new(PidAllocator::new())
    };
}

pub fn pid_alloc() -> PidHandle {
    PID_ALLOCATOR.exclusive_access().alloc()
}

impl Drop for PidHandle {
    fn drop(&mut self) {
        PID_ALLOCATOR.exclusive_access().dealloc(self.0);
    }
}

pub fn kernel_stack_position(pid: usize) -> (usize, usize) {
    let top = TRAMPOLINE - pid * (KERNEL_STACK_SIZE + PAGE_SIZE);
    let bottom = top - KERNEL_STACK_SIZE;
    (bottom, top)
}

pub struct KernelStack {
    pid: usize,
}

impl KernelStack {
    pub fn new(pid_handle: &PidHandle) -> Self {
        let pid = pid_handle.0;
        let (bottom, top) = kernel_stack_position(pid);
        KERNEL_SPACE
            .exclusive_access()
            .insert_framed_area(
                bottom.into(),
                top.into(),
                MapPermission::R | MapPermission::W,
            );
        Self { pid }
    }
    pub fn get_top(&self) -> usize {
        let (_, top) = kernel_stack_position(self.pid);
        top
    }
    #[allow(unused)]
    pub fn push_on_top<T>(&self, value: T) -> *mut T
    where
        T: Sized,
    {
        let top = self.get_top();
        let ptr_mut = (top - core::mem::size_of::<T>()) as *mut T;
        unsafe {
            *ptr_mut = value;
        }
        ptr_mut
    }
}

impl Drop for KernelStack {
    fn drop(&mut self) {
        let (bottom, _) = kernel_stack_position(self.pid);
        let bottom_va: VirtAddr = bottom.into();
        KERNEL_SPACE
            .exclusive_access()
            .remove_area_with_start_vpn(VirtPageNum::from(bottom_va));
    }
}
