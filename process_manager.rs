use vstd::prelude::*;

use vstd::simple_pptr::PointsTo;

use crate::define::*;
use crate::slinkedlist::spec_impl_u::*;
use crate::process::*;
use crate::thread::*;

verus!{

pub struct ProcessManager{
    pub root_proc_ptr: ProcPtr,
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

//scheduler lock: Free, Push(ThreadPtr), Pop(ThreadPtr), Remove(ThreadPtr)
//
//

impl ProcessManager{
    pub open spec fn proc_domain_wf(&self) -> bool {
        &&&
        self.proc_ptrs.wf()
        &&&
        self.proc_ptrs@.to_set() =~= self.proc_perms@.dom()
        &&&
        forall|proc_ptr:ProcPtr| #[auto] self.proc_perms@.dom().contains(proc_ptr) ==> 
            self.proc_perms@[proc_ptr].init()
            &&
            self.proc_perms@[proc_ptr].parent.is_None() <==> self.root_proc_ptr == proc_ptr
            &&
            self.proc_perms@[proc_ptr].children_count == self.proc_perms@[proc_ptr].children@.len()
        &&&
        forall|proc_ptr:ProcPtr| #[auto] self.proc_perms@.dom().contains(proc_ptr) && self.proc_perms@[proc_ptr].parent.is_Some() ==> 
            self.proc_perms@.dom().contains(self.proc_perms@[proc_ptr].parent.unwrap())
            &&
            self.proc_perms@[proc_ptr].parent.unwrap() != proc_ptr
            &&
            self.proc_perms@[self.proc_perms@[proc_ptr].parent].children@.contains(proc_ptr)
        &&&
        forall|proc_ptr:ProcPtr, child_ptr:ProcPtr| #[auto] self.proc_perms@.dom().contains(proc_ptr) && self.proc_perms@[proc_ptr].children@.contains(child_ptr) ==> 
            self.proc_perms@.dom().contains(child_ptr)
            &&
            self.proc_perms@[proc_ptr].child_ptr.parent =~= Some(proc_ptr)

    }
}

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