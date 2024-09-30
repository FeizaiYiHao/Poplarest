use vstd::prelude::*;

use crate::define::*;
use crate::slinkedlist::spec_impl_u::*;

verus!{

pub struct Thread{
    pub owning_proc: ProcPtr,
    pub rev_ptr: SLLIndex,

    pub cpuid_op: Option<CpuId>,
    pub rev_ptr_sched: SLLIndex,
}

}