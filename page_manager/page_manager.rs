use vstd::prelude::*;
verus! {
    use crate::define::*;
    use crate::page_manager::page::*;
    use crate::rwlock::*;
    use crate::array::Array;
    
    pub struct PageManager{
        pub page_array: Array<RWLock<Page, PageLockMajor>, NUM_PAGES>,

        pub container_page_dict: Ghost<Map<ContainerPtr, Set<PageID>>>,
        pub mapped_pages: Ghost<Set<PageID>>,
    }

    impl PageManager{
        pub page_array_wf(&self) -> bool{
            &&&
            self.page_array.wf()
        }

        pub page_state_allocated_wf(&self) -> bool{
            &&&
            forall|page_index:PageID|
                #![trigger self.page_array@[page_index as int]@.owning_container]
                #![trigger self.page_array@[page_index as int]@.state]
                0 <= page_index < NUM_PAGES && self.page_array@[page_index as int]@.state.is_allocated()
                ==>
                self.page_array@[page_index as int]@.owning_container.is_some()
                &&
                self.container_page_dict@.dom().contains(self.page_array@[page_index as int]@.owning_container.unwrap())
                &&
                self.container_page_dict@[self.page_array@[page_index as int]@.owning_container.unwrap()].contains(page_index)
            &&&
            forall|c_ptr:ContainerPtr|
        }
    }
}