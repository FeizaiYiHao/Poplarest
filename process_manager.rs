use vstd::prelude::*;

use vstd::simple_pptr::PointsTo;

use crate::define::*;
use crate::slinkedlist::spec_impl_u::*;
use crate::process::*;

verus!{

pub struct ProcessManager{
    pub proc_ptrs: StaticLinkedList<ProcPtr, MAX_NUM_PROCS>,
    pub proc_perms: Tracked<Map<ProcPtr, PointsTo<Process>>>,

    pub thread_perms: Tracked<Map<ProcPtr, Map<ThreadPtr, PointsTo<Thread>>>>,

    pub scheduler: StaticLinkedList<(ProcPtr, ThreadPtr), MAX_NUM_THREADS>,
}

//Process LinkedList Lock: Free, NewProc, KillProc(ProcPtr)
//NewProc: read locks all parent, rev_ptr, children_count, children.
//KillProc: read locks all parent, rev_ptr. Write locks everything of the process getting killed. 

//Thread Linkedlist Lock: Free, NewThread, KillThread(ThreadPtr)
//NewThread: read locks all parent, rev_ptr,
//KillThread: read locks all parent, rev_ptr,

//scheduler lock: Free, Push(ProcPtr, ThreadPtr), Pop(ProcPtr, ThreadPtr), Remove(ProcPtr, ThreadPtr)
//
//
impl ProcessManager{
    // Add new process perm into map                    --> writelock on process domain (maybe a state that says append only?) 
    // Add new process into linkedlist                  --> writelock on process domain (maybe a state that says append only?) 
    // Set new process's parent to be parent            --> writelock on new process 
    // Set rev_ptr                                      --> writelock on new process 
    // Inc parent's ref counter                         --> writelock on parent
    pub fn new_process(){}

    // Remove process perm from the map (ghost)         --> writelock on process domain (maybe a state that says append only?) 
    // Remove process from the linkedlist               --> writelock on process domain (maybe a state that says append only?) 
    // Dec parent's ref counter                         --> writelock on parent
    pub fn kill_process(){}


    pub fn new_thread(){}
    pub fn kill_thread(){}

}

}