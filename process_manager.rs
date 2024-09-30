use vstd::prelude::*;

use vstd::simple_pptr::PointsTo;

use crate::define::*;
use crate::slinkedlist::spec_impl_u::*;
use crate::process::*;
use crate::thread::*;
use crate::array::*;

verus!{

pub struct ProcessManager{
    pub root_proc_ptr: ProcPtr,
    pub proc_ptrs: StaticLinkedList<ProcPtr, MAX_NUM_PROCS>,
    pub proc_perms: Tracked<Map<ProcPtr, PointsTo<Process>>>,

    pub thread_perms: Tracked<Map<ProcPtr, Map<ThreadPtr, PointsTo<Thread>>>>,

    pub scheduler: StaticLinkedList<(ProcPtr, ThreadPtr), MAX_NUM_THREADS>,

    pub running_threads: Ghost<Map<(ProcPtr, ThreadPtr), CpuId>>,
    pub running_pagetables: Ghost<Set<PageTablePtr>>,
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

    //everything is under the proc domain lock.
    pub open spec fn proc_domain_wf(&self) -> bool {
        &&&
        self.proc_ptrs.wf()
        &&&
        self.proc_ptrs@.to_set() =~= self.proc_perms@.dom()
        &&&
        forall|proc_ptr:ProcPtr| #![auto] self.proc_perms@.dom().contains(proc_ptr) ==> 
            self.proc_perms@[proc_ptr].is_init()
            &&
            self.proc_perms@[proc_ptr].value().parent.is_None() <==> self.root_proc_ptr == proc_ptr
            &&
            self.proc_perms@[proc_ptr].value().children_count == self.proc_perms@[proc_ptr].value().children@.len()
        &&&
        forall|proc_ptr:ProcPtr| #![auto] self.proc_perms@.dom().contains(proc_ptr) && self.proc_perms@[proc_ptr].value().parent.is_Some() ==> 
            self.proc_perms@.dom().contains(self.proc_perms@[proc_ptr].value().parent.unwrap())
            &&
            self.proc_perms@[proc_ptr].value().parent.unwrap() != proc_ptr
            &&
            self.proc_perms@[self.proc_perms@[proc_ptr].value().parent.unwrap()].value().children@.contains(proc_ptr)
        &&&
        forall|proc_ptr:ProcPtr, child_ptr:ProcPtr| #![auto] self.proc_perms@.dom().contains(proc_ptr) && self.proc_perms@[proc_ptr].value().children@.contains(child_ptr) ==> 
            self.proc_perms@.dom().contains(child_ptr)
            &&
            self.proc_perms@[child_ptr].value().parent =~= Some(proc_ptr)
        &&&
        self.proc_perms@.dom() =~= self.thread_perms@.dom()
    }

    pub open spec fn proc_thread_domain_wf(&self, proc_ptr: ProcPtr) -> bool
        recommends
            self.proc_perms@.dom().contains(proc_ptr),
    {
        &&&
        self.proc_perms@[proc_ptr].value().thread_ptrs.wf()
        &&&
        self.thread_perms@[proc_ptr].dom() =~= self.proc_perms@[proc_ptr].value().thread_ptrs@.to_set()
        &&&
        forall|thread_ptr:ThreadPtr|#![auto] self.thread_perms@[proc_ptr].dom().contains(thread_ptr) ==>
            self.thread_perms@[proc_ptr][thread_ptr].is_init()
            &&
            self.thread_perms@[proc_ptr][thread_ptr].value().owning_proc == proc_ptr
    }

    pub open spec fn scheduler_wf(&self) -> bool{
        &&&
        self.scheduler.wf()
        &&&
        forall|proc_ptr: ProcPtr, thread_ptr: ThreadPtr| #![auto] self.scheduler@.to_set().contains((proc_ptr, thread_ptr)) ==>
            self.thread_perms@[proc_ptr][thread_ptr].value().cpuid_op.is_None()
        &&&
        forall|proc_ptr: ProcPtr, thread_ptr: ThreadPtr| #![auto] self.proc_perms@.dom().contains(proc_ptr) && self.thread_perms@[proc_ptr][thread_ptr].value().cpuid_op.is_None() ==>
            self.scheduler@.to_set().contains((proc_ptr, thread_ptr))
    }

    pub open spec fn running_threads_wf(&self) -> bool {
        &&&
        forall|proc_ptr: ProcPtr, thread_ptr: ThreadPtr| #![auto] self.running_threads@.dom().contains((proc_ptr, thread_ptr)) ==>
            self.thread_perms@[proc_ptr][thread_ptr].value().cpuid_op =~= Some(self.running_threads@[(proc_ptr, thread_ptr)])
        &&&
        forall|proc_ptr: ProcPtr, thread_ptr: ThreadPtr| #![auto] self.proc_perms@.dom().contains(proc_ptr) && self.thread_perms@[proc_ptr][thread_ptr].value().cpuid_op.is_Some() ==>
            self.running_threads@.dom().contains((proc_ptr, thread_ptr)) && self.thread_perms@[proc_ptr][thread_ptr].value().cpuid_op =~= Some(self.running_threads@[(proc_ptr, thread_ptr)])
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