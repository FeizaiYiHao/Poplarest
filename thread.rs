use vstd::prelude::*;

use crate::define::*;
use crate::slinkedlist::spec_impl_u::*;

verus!{

pub struct Thread{
    pub parent: ProcPtr,
    pub rev_ptr: SLLIndex,

    pub scheduled: bool,
    pub rev_ptr_sched: SLLIndex,
}

}