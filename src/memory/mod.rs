use x86_64::{
    structures::paging::{PageTable, OffsetPageTable, Translate},
    VirtAddr, PhysAddr
};

static mut PAGE_TABLE: Option<OffsetPageTable<'static>> = Option::None;

pub unsafe fn init(physical_memory_offset: VirtAddr) {
    PAGE_TABLE = Some(OffsetPageTable::new(active_level_4_table(physical_memory_offset), physical_memory_offset));
}

unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();
    let physical_address = level_4_table_frame.start_address();
    let virtual_address = physical_memory_offset + physical_address.as_u64();
    
    &mut *virtual_address.as_mut_ptr()
}

pub unsafe fn translate_address(address: VirtAddr) -> Option<PhysAddr> {
    match &PAGE_TABLE {
        None => None,
        Some(page_table) => page_table.translate_addr(address)
    }
}