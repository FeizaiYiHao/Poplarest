use vstd::prelude::*;
use core::sync::atomic::AtomicBool;

verus! {

use crate::define::*;
use crate::lock_agent::*;
use crate::lemma::lemma_u::*;
// use crate::lemma::lemma_t::*;

pub struct RWLock<T, const N: usize>{
    pub locked: AtomicBool,
    pub num_writer:usize,
    pub num_readers: usize,

    pub data: T,

    pub writing_thread: Ghost<Option<ThreadID>>,
    pub reading_threads: Ghost<Set<ThreadID>>,
}

impl <T: Clone, const N: usize> Clone for RWLock<T, N>{
    fn clone(&self) -> Self {
        Self{
            locked: AtomicBool::new(false),
            num_writer: self.num_writer,
            num_readers: self.num_readers,
            data: self.data.clone(),
            writing_thread: self.writing_thread,
            reading_threads: self.reading_threads,
        }
    }
}

pub tracked struct ReadPerm {
} 

impl ReadPerm{
    pub spec fn lock_id(self) -> int;
}

pub tracked struct WritePerm {
}

impl WritePerm{
    pub spec fn lock_id(self) -> int;
}

impl<T,const N: usize> RWLock<T, N>{
    pub spec fn lock_id(self) -> int;

    pub spec fn separate(self) -> bool;

    pub open spec fn lock_id_pair(self) -> LockIDPair{
        (self.lock_major(), self.lock_minor())
    }

    pub open spec fn lock_major(self) -> LockMajorID{
        N
    }

    pub open spec fn lock_minor(self) -> LockMinorID{
        0
    }

    pub open spec fn writing_thread(self) -> Option<ThreadID>{
        self.writing_thread@
    }

    pub open spec fn reading_threads(self) -> Set<ThreadID>{
        self.reading_threads@
    }

    pub open spec fn view(self) -> T{
        self.data
    }

    #[inline(always)]
    #[verifier::external_body]
    pub fn borrow<'a>(self, Tracked(lock_agent): Tracked<&'a ReadPerm>) -> (v: &'a T)
        requires
            self.lock_id() == lock_agent.lock_id(),
        ensures
            *v == self@,
    {
        unsafe { &* (&self.data as *const T) }
    }

    #[verifier(external_body)]
    pub fn read_lock(&mut self, Tracked(lock_agent): Tracked<&mut LockAgent>) -> (ret:Tracked<ReadPerm>)
        requires
            old(self).reading_threads().contains(old(lock_agent).thread_id) == false,
            old(self).writing_thread().is_None() || old(self).writing_thread().unwrap() != old(lock_agent).thread_id,
            step_lock_aquire_requires(old(lock_agent), old(self).lock_id_pair()),
        ensures
            self.reading_threads() =~= old(self).reading_threads().insert(lock_agent.thread_id),
            old(self).writing_thread().is_None(),
            self.writing_thread() =~= old(self).writing_thread(),
            old(self)@ == self@,
            step_lock_aquire_ensures(old(lock_agent), lock_agent, old(self).lock_id_pair()),
            old(self).lock_id_pair() =~= self.lock_id_pair(),
            self.separate() == false,
            self.lock_id() == old(self).lock_id(),
            ret@.lock_id() == self.lock_id(),
    {
        Tracked::assume_new()
    }

    #[verifier(external_body)]
    pub fn read_unlock(&mut self, Tracked(lock_agent): Tracked<&mut LockAgent>, Tracked(read_perm):Tracked<ReadPerm>)
        requires
            old(self).reading_threads().contains(old(lock_agent).thread_id),
            step_lock_release_requires(old(lock_agent), old(self).lock_id_pair()),
            read_perm.lock_id() == old(self).lock_id(),
        ensures
            self.reading_threads() =~= old(self).reading_threads().remove(lock_agent.thread_id),
            self.writing_thread() =~= old(self).writing_thread(),
            old(self)@ == self@,
            step_lock_release_ensures(old(lock_agent), lock_agent, old(self).lock_id_pair()),
            old(self).lock_id_pair() =~= self.lock_id_pair(),
            self.separate() == false,
            self.lock_id() == old(self).lock_id(),
    {
    }

    #[verifier(external_body)]
    pub fn new(data: T) -> (ret:Self)
        ensures
            ret.writing_thread().is_None(),
            ret.reading_threads() =~= Set::empty(),
            ret@ =~= data,
    {
        Self{
            locked: AtomicBool::new(false),
            num_writer:0,
            num_readers: 0,
        
            data: data,
        
            writing_thread: Ghost(None),
            reading_threads: Ghost(Set::empty()),
        }
    }
}

pub fn test(Tracked(lock_agent): Tracked<&mut LockAgent>,l1 :&mut RWLock<usize, 1>, l2 :&mut RWLock<usize, 2>)
    requires
        old(l1).reading_threads().contains(old(lock_agent).thread_id) == false,
        old(l1).writing_thread().is_None() || old(l1).writing_thread().unwrap() != old(lock_agent).thread_id,
        old(l2).reading_threads().contains(old(lock_agent).thread_id) == false,
        old(l2).writing_thread().is_None() || old(l2).writing_thread().unwrap() != old(lock_agent).thread_id,
        old(lock_agent).wf(),
        old(lock_agent).is_empty(),
    ensures
        lock_agent.is_empty(),
{
    proof{
        seq_push_lemma::<LockIDPair>();
        seq_remove_lemma_2::<LockIDPair>();
    }
    let read_perm_l1 = l1.read_lock(Tracked(lock_agent));
    let read_perm_l2 = l2.read_lock(Tracked(lock_agent));
    assert(lock_agent.lock_seq.len() == 2);
    l1.read_unlock(Tracked(lock_agent),read_perm_l1);
    assert(lock_agent.lock_seq.len() == 1);
    l2.read_unlock(Tracked(lock_agent),read_perm_l2);
    assert(lock_agent.lock_seq.len() == 0);
}

}