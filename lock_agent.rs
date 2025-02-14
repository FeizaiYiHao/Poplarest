use vstd::prelude::*;

verus! {

use crate::define::*;

pub open spec fn spec_lock_id_greater_than(p1: LockIDPair, p2: LockIDPair) -> bool{
    ||| p1.0 < p2.0 
    ||| p1.0 == p2.0 && p1.1 < p2.1
}

#[verifier(when_used_as_spec(spec_lock_id_greater_than))]
pub fn lock_id_greater_than(p1: LockIDPair, p2: LockIDPair) -> (ret:bool)
    ensures
        ret == lock_id_greater_than(p1, p2)
{
    if p1.0 < p2.0 {
        return true;
    }else if p1.0 == p2.0 && p1.1 < p2.1{
        return true;
    }else{
        return false;
    }
}

pub tracked struct LockAgent {
    pub thread_id: ThreadID,
    pub lock_seq: Seq<LockIDPair>    
} 

impl LockAgent{

    pub open spec fn wf(&self) -> bool{
        &&&
        self.lock_seq.no_duplicates()
        &&&
        forall|i:int| #![trigger self.lock_seq[i]] 0<=i<self.lock_seq.len()-1 ==> lock_id_greater_than(self.lock_seq[i], self.lock_seq[i+1])
    }

    pub open spec fn is_empty(&self) -> bool{
        &&&
        self.lock_seq =~= Seq::empty()
    }
}

pub open spec fn step_lock_aquire_requires(old: &LockAgent, lock_id:LockIDPair) -> bool{
    &&&
    old.wf()
    &&&
    old.lock_seq =~= Seq::empty() || lock_id_greater_than(old.lock_seq.last(), lock_id)
}

pub open spec fn step_lock_aquire_ensures(old: &LockAgent, new: &LockAgent, lock_id:LockIDPair) -> bool{
    &&&
    new.wf()
    &&&
    new.lock_seq =~= old.lock_seq.push(lock_id)
    &&&
    new.thread_id =~= old.thread_id
}

pub open spec fn step_lock_release_requires(old: &LockAgent, lock_id:LockIDPair) -> bool{
    &&&
    old.wf()
    &&&
    old.lock_seq.contains(lock_id)
}

pub open spec fn step_lock_release_ensures(old: &LockAgent, new: &LockAgent, lock_id:LockIDPair) -> bool{
    &&&
    new.wf()
    &&&
    new.lock_seq =~= old.lock_seq.remove_value(lock_id)
    &&&
    new.lock_seq.len() == old.lock_seq.len() - 1
    &&&
    new.thread_id =~= old.thread_id
}

}