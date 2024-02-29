use libc::{mprotect, PROT_READ, PROT_WRITE, _SC_PAGE_SIZE};
use log::info;
use crate::utils::logger::dbg_info;
use plt_rs::{PltError, PltResult};
use std::ffi::{c_uint,c_void,};

fn sign_extend32(data: u32, size: u32) -> i32 {
    assert!(size > 0 && size <= 32);
    ((data << (32 - size)) as i32) >> (32 - size)
}

pub fn get_branch_addr_from_inst_addr(insn_addr: *mut *const c_void) -> *const c_void {
    // 0000f1ac  abecff17   b       sub_a458
    // We get 0xf1ac as fnc_addr, first check if it's a branch instruction then calculate the branch address
    // Get 4 bytes from the function address
    let insn_bytes = unsafe { std::slice::from_raw_parts(insn_addr as *const c_uint, 1) };
    // Check if it's a branch instruction
    dbg_info!("insn_bytes: {:x}", insn_bytes[0]);
    //17ffeca
    let insn = insn_bytes[0];
    let is_b = (insn & 0x14000000) == 0x14000000;
    if is_b {
        let offset = (insn & 0xebffffff) * 4;
        // dbg_info!("Offset: {:x}", offset as i32);
        let extended_offset = sign_extend32(offset, 26);
        dbg_info!("insn_addr: {:p}", insn_addr);
        dbg_info!("Extended offset: {:x}", extended_offset);
        let branch_addr =
            (insn_addr as *const u8).wrapping_offset(extended_offset as isize) as *const c_void;
        dbg_info!("Branch address: {:p}", branch_addr);
        branch_addr
    } else {
        dbg_info!("Not a branch instruction");
        insn_addr as *const c_void
    }
}

pub fn replace_address(
    func_ptr: *mut *const c_void,
    destination: *const c_void,
) -> PltResult<*const c_void> {
    let page_size = unsafe { libc::sysconf(_SC_PAGE_SIZE) as usize };
    let new_func_ptr = get_branch_addr_from_inst_addr(func_ptr);
    let recalc_func_ptr;
    if new_func_ptr != 0 as *const c_void {
        recalc_func_ptr = new_func_ptr as *mut *const c_void;
    } else {
        recalc_func_ptr = func_ptr;
    }
    dbg_info!("func ptr : {:p}", recalc_func_ptr);

    let aligned_address = ((recalc_func_ptr as usize / page_size) * page_size) as *mut c_void;

    unsafe {
        // Set the memory page to read, write
        let prot_res = mprotect(aligned_address, page_size, PROT_WRITE | PROT_READ);
        if prot_res != 0 {
            return Err(PltError::Protection(
                recalc_func_ptr as *mut _,
                aligned_address,
                PROT_READ | PROT_WRITE,
                std::io::Error::last_os_error().raw_os_error().unwrap(),
            ));
        }

        dbg_info!(
            "Replacing function pointer {:p} with {:p}",
            recalc_func_ptr, destination
        );
        let previous_address = std::ptr::replace(recalc_func_ptr, destination);
        dbg_info!("Previous address: {:p}", previous_address);

        // Set the memory page protection back to read only
        let prot_res = mprotect(aligned_address, page_size, PROT_READ);
        if prot_res != 0 {
            return Err(PltError::Protection(
                recalc_func_ptr as *mut _,
                aligned_address,
                PROT_READ,
                std::io::Error::last_os_error().raw_os_error().unwrap(),
            ));
        }

        Ok(previous_address as *const c_void)
    }
}
