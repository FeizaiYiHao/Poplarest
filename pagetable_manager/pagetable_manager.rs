use vstd::prelude::*;
verus! {
    use crate::define::*;
    pub ghost struct PagetableGhost{
        pub map_4k: Map<VAddr, PageID>,
        pub writing_thread: Option<ThreadID>,
    }
    pub ghost struct PagetableManagerGhost{
        pub pagetable_dict: Map<PagetableID, PagetableGhost>,
    }

    pub open spec fn write_lock_aquires(old: PagetableManagerGhost, pagetable_id:PagetableID) -> bool
    {
        &&&
        old.pagetable_dict[pagetable_id].writing_thread.is_None()
    }

    pub open spec fn write_lock_ensures(old: PagetableManagerGhost, new: PagetableManagerGhost, pagetable_id:PagetableID, thread_id:ThreadID) -> bool
    {
        &&&
        new.pagetable_dict[pagetable_id].writing_thread == Some(thread_id)
        &&&
        old.pagetable_dict.dom() =~= new.pagetable_dict.dom()
        &&&
        forall|pt_id:PagetableID| #![auto] old.pagetable_dict.dom().contains(pt_id) && pt_id != pagetable_id ==> 
            old.pagetable_dict[pt_id] == new.pagetable_dict[pt_id]
        &&&
        old.pagetable_dict[pagetable_id].map_4k == new.pagetable_dict[pagetable_id].map_4k
    }

    impl PagetableManagerGhost{

        
        #[verifier(external_body)]
        pub fn write_lock(&mut self, pagetable_id:PagetableID, thread_id:ThreadID)
            requires 
                write_lock_aquires(*old(self), pagetable_id),
            ensures
                write_lock_ensures(*old(self), *self, pagetable_id, thread_id),
        {}
    }
}