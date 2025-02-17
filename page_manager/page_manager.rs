use vstd::prelude::*;
verus! {
    use crate::define::*;
    use crate::page_manager::page::*;
    use crate::rwlock::*;
    use crate::array::Array;
    
    pub struct PageManager{
        pub page_array: Array<RWLock<Page, PageLockMajor>, NUM_PAGES>,

        pub container_page_4k_dict: Ghost<Map<ContainerPtr, Set<PageID>>>,
        pub container_page_2m_dict: Ghost<Map<ContainerPtr, Set<PageID>>>,
        pub container_page_1g_dict: Ghost<Map<ContainerPtr, Set<PageID>>>,
        pub mapped_pages: Ghost<Set<PageID>>,
    }

    impl PageManager{
        pub open spec fn page_array_wf(&self) -> bool{
            &&&
            self.page_array.wf()
        }

        pub open spec fn page_state_allocated_wf(&self) -> bool{
            &&&
            forall|page_id:PageID|
                #![trigger self.page_array@[page_id as int]@.owning_container]
                #![trigger self.page_array@[page_id as int]@.state]
                0 <= page_id < NUM_PAGES && self.page_array@[page_id as int]@.state =~= PageState::Allocated
                ==>
                self.page_array@[page_id as int]@.owning_container.is_some()
                &&
                self.container_page_4k_dict@.dom().contains(self.page_array@[page_id as int]@.owning_container.unwrap())
                &&
                self.container_page_4k_dict@[self.page_array@[page_id as int]@.owning_container.unwrap()].contains(page_id)
            &&&
            forall|c_ptr:ContainerPtr, page_id:PageID|
                self.container_page_4k_dict@.dom().contains(c_ptr) && self.container_page_4k_dict@[c_ptr].contains(page_id)
                ==>
                0 <= page_id < NUM_PAGES && self.page_array@[page_id as int]@.state =~= PageState::Allocated4k
        }
    }
}