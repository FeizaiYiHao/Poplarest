use vstd::prelude::*;

use crate::define::*;
use crate::slinkedlist::spec_impl_u::*;

verus!{

pub struct Process{
    pub parent: Option<ProcPtr>,
    pub children_count: usize,
    pub children: Ghost<Set<ProcPtr>>,
    pub thread_ptrs: StaticLinkedList<ThreadPtr, MAX_NUM_THREADS_PER_PROC>,
    pub pagetable_ptr: PageTablePtr, //controled by proc domain as well.
    pub rev_ptr: SLLIndex,
}

}