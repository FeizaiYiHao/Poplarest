use vstd::prelude::*;
use core::sync::atomic::AtomicBool;

verus! {

#[allow(inconsistent_fields)]
pub enum ProcDomainWriterState{
    NoWriter,
    LockedNewProc { new_proc_ptr: ProcPtr },
    LockedKillProc { kill_proc_ptr: ProcPtr},
}

pub struct ProcDomainLock{
    locked: AtomicBool,
    pub writer_state:ProcDomainWriterState,
    pub num_readers: usize,
}

pub struct ProcDomainLocked<T>{
    inner: T
}

impl ProcDomainLocked<T>{
    
}

}