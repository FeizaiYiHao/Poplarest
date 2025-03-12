use vstd::prelude::*;

verus! {
use crate::define::*;
use crate::pagetable_manager::pagetable_manager::*;
use crate::page_manager::page_manager::*;

pub ghost struct ReferenceCounterStateMachine{
    pub page_manager: PageManagerGhost,
    pub pagetable_manager: PagetableManagerGhost,
}

impl ReferenceCounterStateMachine{

    // true all the time
    pub open spec fn page_array_inv(&self) -> bool{
        &&&
        self.page_manager.page_array.len() == NUM_PAGES
    }

    pub open spec fn dom_match(&self) -> bool{
        &&&
forall|p_id:PageID, pt_id:PagetableID| 
    #![auto] 
    0 <= p_id < NUM_PAGES && self.page_manager.page_array[p_id as int].mappings_4k.dom().contains(pt_id) ==>
            self.pagetable_manager.pagetable_dict.dom().contains(pt_id) 
        &&&
        forall|pt_id:PagetableID, va:VAddr| 
            #![auto] 
            self.pagetable_manager.pagetable_dict.dom().contains(pt_id) && 
            self.pagetable_manager.pagetable_dict[pt_id].map_4k.contains_key(va) ==>
                0 <= self.pagetable_manager.pagetable_dict[pt_id].map_4k[va] < NUM_PAGES 
    }

    pub open spec fn mapping_match(&self) -> bool{
        &&&
        forall|p_id:PageID, pt_id:PagetableID, va:VAddr| 
            #![auto] 
            0 <= p_id < NUM_PAGES && self.page_manager.page_array[p_id as int].mappings_4k.dom().contains(pt_id) &&
                self.page_manager.page_array[p_id as int].mappings_4k[pt_id].contains(va) ==>
                (
                    self.pagetable_manager.pagetable_dict[pt_id].map_4k.dom().contains(va)
                    &&
                    self.pagetable_manager.pagetable_dict[pt_id].map_4k[va] == p_id
                    ||
                    (
                        self.pagetable_manager.pagetable_dict[pt_id].writing_thread.is_Some()
                        &&
                        self.page_manager.page_array[p_id as int].writing_thread.is_Some()
                        &&
                        self.pagetable_manager.pagetable_dict[pt_id].writing_thread.unwrap() == 
                            self.page_manager.page_array[p_id as int].writing_thread.unwrap()
                    )
                )
        &&&
        forall|p_id:PageID, pt_id:PagetableID, va:VAddr| 
            #![auto] 
            self.pagetable_manager.pagetable_dict.dom().contains(pt_id) &&
                self.pagetable_manager.pagetable_dict[pt_id].map_4k.contains_key(va) &&
                self.pagetable_manager.pagetable_dict[pt_id].map_4k[va] == p_id ==>
                (
                    self.page_manager.page_array[p_id as int].mappings_4k[pt_id].contains(va)
                    ||
                    (
                        self.pagetable_manager.pagetable_dict[pt_id].writing_thread.is_Some()
                        &&
                        self.page_manager.page_array[p_id as int].writing_thread.is_Some()
                        &&
                        self.pagetable_manager.pagetable_dict[pt_id].writing_thread.unwrap() == 
                            self.page_manager.page_array[p_id as int].writing_thread.unwrap()
                    )
                )
    }


    pub fn write_lock_page(&mut self, page_id: PageID, thread_id: ThreadID)
    {

    }
}

pub open spec fn write_lock_page_require(old: ReferenceCounterStateMachine, page_id: PageID, thread_id: ThreadID) -> bool{
    &&&
    old.page_manager.page_array[page_id as int].writing_thread.is_None()
}

pub open spec fn write_lock_page_ensure(old: ReferenceCounterStateMachine, new: ReferenceCounterStateMachine, page_id: PageID, thread_id: ThreadID) -> bool{
    &&&
    old.page_manager.page_array[page_id as int].writing_thread == Some(thread_id)
    &&&
    old.pagetable_manager =~= new.pagetable_manager
    &&&
    forall|p_id:PageID| #![auto] 0 <= p_id < NUM_PAGES && page_id != p_id ==> 
        old.page_manager.page_array[p_id as int] == new.page_manager.page_array[p_id as int]
    &&&
    old.page_manager.page_array[page_id as int].mappings_4k == new.page_manager.page_array[page_id as int].mappings_4k
}

pub open spec fn write_unlock_page_require(old: ReferenceCounterStateMachine, page_id: PageID, thread_id: ThreadID) -> bool{
    &&&
    old.page_manager.page_array[page_id as int].writing_thread.is_Some()
    &&&
    old.page_manager.page_array[page_id as int].writing_thread.unwrap() == thread_id
}

pub open spec fn write_unlock_page_ensure(old: ReferenceCounterStateMachine, new: ReferenceCounterStateMachine, page_id: PageID, thread_id: ThreadID) -> bool{
    &&&
    old.page_manager.page_array[page_id as int].writing_thread.is_None()
    &&&
    old.pagetable_manager =~= new.pagetable_manager
    &&&
    forall|p_id:PageID| #![auto] 0 <= p_id < NUM_PAGES && page_id != p_id ==> 
        old.page_manager.page_array[p_id as int] == new.page_manager.page_array[p_id as int]
    &&&
    old.page_manager.page_array[page_id as int].mappings_4k == new.page_manager.page_array[page_id as int].mappings_4k
}

pub open spec fn add_mapping_aquire(old: ReferenceCounterStateMachine, page_id: PageID, pagetable_id: PagetableID, va:VAddr, thread_id: ThreadID) -> bool{
    &&&
    0 <= page_id < NUM_PAGES
    &&&
    old.page_manager.page_array[page_id as int].writing_thread.is_Some()
    &&&
    old.page_manager.page_array[page_id as int].writing_thread.unwrap() == thread_id
    &&&
    old.pagetable_manager.pagetable_dict[pagetable_id].writing_thread =~= Some(thread_id)
    &&&
    old.page_manager.page_array[page_id as int].mappings_4k[pagetable_id].contains(va) == false
    &&&
    old.pagetable_manager.pagetable_dict[pagetable_id].map_4k.dom().contains(va) == false
}

pub open spec fn add_mapping_ensure(old: ReferenceCounterStateMachine, new: ReferenceCounterStateMachine, page_id: PageID, pagetable_id: PagetableID, va:VAddr, thread_id: ThreadID) -> bool{
    &&&
    old.page_manager.page_array[page_id as int].mappings_4k[pagetable_id].insert(va) =~= new.page_manager.page_array[page_id as int].mappings_4k[pagetable_id]
    &&&
    old.page_manager.page_array[page_id as int].mappings_4k.dom() =~= new.page_manager.page_array[page_id as int].mappings_4k.dom()
    &&&
    old.pagetable_manager.pagetable_dict.dom() =~= new.pagetable_manager.pagetable_dict.dom()
    &&&
    old.pagetable_manager.pagetable_dict[pagetable_id].map_4k.insert(va, page_id) =~= new.pagetable_manager.pagetable_dict[pagetable_id].map_4k
    &&&
    forall|pt_id:PagetableID| #![auto] old.pagetable_manager.pagetable_dict.contains_key(pt_id) && pt_id != pagetable_id ==> 
        new.pagetable_manager.pagetable_dict[pt_id] == old.pagetable_manager.pagetable_dict[pt_id]
        &&
        old.page_manager.page_array[page_id as int].mappings_4k[pt_id] =~= new.page_manager.page_array[page_id as int].mappings_4k[pt_id]
    &&&
    forall|p_id:PageID| #![auto] 0 <= p_id < NUM_PAGES && p_id != page_id ==> 
        old.page_manager.page_array[p_id as int] =~= new.page_manager.page_array[p_id as int]
    &&&
    old.pagetable_manager.pagetable_dict[pagetable_id].writing_thread =~= new.pagetable_manager.pagetable_dict[pagetable_id].writing_thread
    &&&
    old.page_manager.page_array[page_id as int].writing_thread =~= new.page_manager.page_array[page_id as int].writing_thread
    &&&
    old.page_manager.page_array.len() =~= new.page_manager.page_array.len()
}

pub open spec fn remove_pagetable_require(old: ReferenceCounterStateMachine, pagetable_id: PagetableID, thread_id: ThreadID) -> bool{
    &&&
    old.pagetable_manager.pagetable_dict[pagetable_id].writing_thread =~= Some(thread_id)
    &&&
    old.pagetable_manager.pagetable_dict[pagetable_id].map_4k.dom() == Set::<VAddr>::empty()
    &&&
    forall|p_id:PageID| #![auto] 0 <= p_id < NUM_PAGES ==> 
        (old.page_manager.page_array[p_id as int].mappings_4k.dom().contains(pagetable_id) == false)
        ||
        (old.page_manager.page_array[p_id as int].mappings_4k[pagetable_id] =~= Set::<VAddr>::empty())
}

pub open spec fn remove_pagetable_ensure(old: ReferenceCounterStateMachine, new: ReferenceCounterStateMachine, pagetable_id: PagetableID, thread_id: ThreadID) -> bool{
    &&&
    old.pagetable_manager.pagetable_dict.remove(pagetable_id) =~= new.pagetable_manager.pagetable_dict
    &&&
    old.page_manager.page_array.len() =~= new.page_manager.page_array.len()
    &&&
    forall|p_id:PageID| #![auto] 0 <= p_id < NUM_PAGES ==> 
        new.page_manager.page_array[p_id as int].mappings_4k =~= old.page_manager.page_array[p_id as int].mappings_4k.remove(pagetable_id)
    &&&
    forall|p_id:PageID| #![auto] 0 <= p_id < NUM_PAGES ==> 
        old.page_manager.page_array[p_id as int].writing_thread =~= new.page_manager.page_array[p_id as int].writing_thread
    &&&
    forall|pt_id:PagetableID| #![auto] new.pagetable_manager.pagetable_dict.dom().contains(pt_id) ==> 
        old.pagetable_manager.pagetable_dict[pt_id].writing_thread =~= new.pagetable_manager.pagetable_dict[pt_id].writing_thread
}

pub proof fn add_mapping_inv(old: ReferenceCounterStateMachine, new: ReferenceCounterStateMachine, page_id: PageID, pagetable_id: PagetableID, va:VAddr, thread_id: ThreadID)
    requires
        old.page_array_inv(),
        old.dom_match(),
        old.mapping_match(),
        add_mapping_aquire(old, page_id, pagetable_id, va, thread_id),
        add_mapping_ensure(old, new, page_id, pagetable_id, va, thread_id),
    ensures
        new.page_array_inv(),
        new.dom_match(),
        new.mapping_match(), 
{
}

pub proof fn remove_pagetable_inv(old: ReferenceCounterStateMachine, new: ReferenceCounterStateMachine, pagetable_id: PagetableID, thread_id: ThreadID)
    requires
        old.page_array_inv(),
        old.dom_match(),
        old.mapping_match(),
        remove_pagetable_require(old, pagetable_id, thread_id),
        remove_pagetable_ensure(old, new, pagetable_id, thread_id),
    ensures
        new.page_array_inv(),
        new.dom_match(),
        new.mapping_match(), 
{

    assert(
        forall|p_id:PageID, pt_id:PagetableID, va:VAddr| 
        #![auto] 
        0 <= p_id < NUM_PAGES && new.page_manager.page_array[p_id as int].mappings_4k.dom().contains(pt_id) &&
            new.page_manager.page_array[p_id as int].mappings_4k[pt_id].contains(va) ==>
            (
                new.pagetable_manager.pagetable_dict[pt_id].map_4k.dom().contains(va)
                &&
                new.pagetable_manager.pagetable_dict[pt_id].map_4k[va] == p_id
                ||
                (
                    new.pagetable_manager.pagetable_dict[pt_id].writing_thread.is_Some()
                    &&
                    new.page_manager.page_array[p_id as int].writing_thread.is_Some()
                    &&
                    new.pagetable_manager.pagetable_dict[pt_id].writing_thread.unwrap() == 
                    new.page_manager.page_array[p_id as int].writing_thread.unwrap()
                )
            )
        );

    assert(
    forall|p_id:PageID, pt_id:PagetableID, va:VAddr| 
        #![auto] 
        new.pagetable_manager.pagetable_dict.dom().contains(pt_id) &&
        new.pagetable_manager.pagetable_dict[pt_id].map_4k.contains_key(va) &&
            new.pagetable_manager.pagetable_dict[pt_id].map_4k[va] == p_id ==>
            (
                new.page_manager.page_array[p_id as int].mappings_4k[pt_id].contains(va)
                ||
                (
                    new.pagetable_manager.pagetable_dict[pt_id].writing_thread.is_Some()
                    &&
                    new.page_manager.page_array[p_id as int].writing_thread.is_Some()
                    &&
                    new.pagetable_manager.pagetable_dict[pt_id].writing_thread.unwrap() == 
                    new.page_manager.page_array[p_id as int].writing_thread.unwrap()
                )
            )
        );
}

}