use core::convert::TryInto;

use crate::temp_panic;

const PML4_BASE: u64 = 0x1000;

pub enum AddressType {
    VirtualAddress(VirtualAddress),
    PhysicalAddress(*const u64),
}

pub struct VirtualAddress{
    pub vaddr: *const u64,
    pub pml4_index: usize,
    pub pml3_index: usize,
    pub pml2_index: usize,
    pub pml1_index: usize,
    pub page_offset: usize
}

impl VirtualAddress {
    pub fn new(addr: u64) -> Self{
        let virt: usize = addr.try_into().unwrap();
        VirtualAddress { 
            vaddr: addr as *const u64,
            pml4_index: virt >> 39,
            pml3_index: (virt >> 30) & 0b111111111,
            pml2_index: (virt >> 21) & 0b111111111,
            pml1_index: (virt >> 12) & 0b111111111,
            page_offset: virt & 0xFFF
        }
    }
}

pub struct Mapper {
    pub pml4_addr: *const u64,
    
}

pub enum MapError{
    AlreadyMapped,
    OutOfRange,
    NotAligned
}

impl Mapper{
    pub fn default() -> Self{

        Mapper {
            pml4_addr: PML4_BASE as *const u64
        }
    }

    pub fn info_table(&self) {

        todo!()
    }

    pub fn map_v_p(&self, virt_addr: AddressType, phy_addr: AddressType) -> Result<(), MapError>{
        if let (AddressType::VirtualAddress(vaddr), AddressType::PhysicalAddress(paddr)) = (virt_addr, phy_addr) {
            if self.pml4_addr.is_null() { temp_panic("Invalid Page Table Address") }
            if vaddr.pml4_index > 0 || vaddr.pml3_index > 7 { return Err(MapError::OutOfRange); }
                
        } else {
            panic!()
        }
        todo!()
    }

    pub fn resolve(&self, virtual_address: AddressType) -> AddressType{
        unsafe{

            if let AddressType::VirtualAddress(addr) = virtual_address {
                if self.pml4_addr.is_null() { temp_panic("l4 panic") }
                let p3_address = (self.pml4_addr.wrapping_add(addr.pml4_index).read() & 0xFFFFFFFFFFFFF000) as *const u64;
                if p3_address.is_null() { temp_panic("l3 panic") }
                let p2_address = (p3_address.wrapping_add(addr.pml3_index).read() & 0xFFFFFFFFFFFFF000) as *const u64;
                if p2_address.is_null() { temp_panic("l2 panic") }
                let p1_address = (p2_address.wrapping_add(addr.pml2_index).read() & 0xFFFFFFFFFFFFF000) as *const u64;
                if p1_address.is_null() { temp_panic("l1 panic") }
                let page_address = (p1_address.wrapping_add(addr.pml1_index).read() & 0xFFFFFFFFFFFFF000) as *const u64;
                return AddressType::PhysicalAddress((page_address as *const u8).wrapping_add(addr.page_offset) as *const u64);
            } else {
                panic!("Attempt to resolve physical address");
            }
        }
    }

}


