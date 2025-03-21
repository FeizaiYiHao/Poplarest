use vstd::prelude::*;
use core::mem::MaybeUninit;

verus! {
use vstd::simple_pptr::*;
use crate::define::*;
use crate::lemma::lemma_u::*;

pub struct DLLNode<V> {
    pub prev: Option<DLLNodePointer>,
    pub next: Option<DLLNodePointer>,
    pub payload: V,
}

#[verifier(external_body)]
fn set_prev<V>(ptr:DLLNodePointer, Tracked(perm): Tracked< &mut PointsTo<DLLNode<V>>>, v: Option<DLLNodePointer>)
    requires
        old(perm).addr() == ptr,
        old(perm).is_init(),
    ensures
        old(perm).addr() == perm.addr(),
        perm.is_init(),
        perm.value().prev == v,
        perm.value().next == old(perm).value().next,
        perm.value().payload == old(perm).value().payload,
{
    unsafe{
        let uptr = ptr as *mut MaybeUninit<DLLNode<V>>;
        (*uptr).assume_init_mut().prev = v;
    }
}

#[verifier(external_body)]
fn set_next<V>(ptr:DLLNodePointer, Tracked(perm): Tracked< &mut PointsTo<DLLNode<V>>>, v: Option<DLLNodePointer>)
    requires
        old(perm).addr() == ptr,
        old(perm).is_init(),
    ensures
        old(perm).addr() == perm.addr(),
        perm.is_init(),
        perm.value().next == v,
        perm.value().prev == old(perm).value().prev,
        perm.value().payload == old(perm).value().payload,
{
    unsafe{
        let uptr = ptr as *mut MaybeUninit<DLLNode<V>>;
        (*uptr).assume_init_mut().next = v;
    }
}

pub struct DLL<V>{
    head: Option<DLLNodePointer>,
    tail: Option<DLLNodePointer>,
    ptrs_seq: Ghost<Seq<DLLNodePointer>>,
    value_seq: Ghost<Seq<V>>,
    len: usize,

    perms: Tracked<Map<DLLNodePointer, PointsTo<DLLNode<V>>>>
}

impl<V> DLL<V>{
    #[verifier(when_used_as_spec(spec_len))]
    pub fn len(&self) -> (ret:usize)
        ensures
            self.len() == ret,
    {
        self.len
    }
    pub closed spec fn spec_len(&self) -> usize{
        self.len
    }   
    pub closed spec fn view(&self) -> Seq<V>{
        self.value_seq@
    }
    pub closed spec fn closure(&self) -> Set<DLLNodePointer>{
        self.ptrs_seq@.to_set()
    }
    pub closed spec fn resolve(&self, n_ptr:DLLNodePointer) -> V
        recommends
            self.closure().contains(n_ptr),
    {
        self.perms@[n_ptr].value().payload
    }
    pub closed spec fn wf_len(&self) -> bool{
        &&&
        self.len == self.ptrs_seq@.len()
        &&&
        self.len == self.value_seq@.len()
        &&&
        self.len == 0 <==> self.head.is_None() && self.tail.is_None()
        &&&
        self.head.is_None() == self.tail.is_None()
    }
    pub closed spec fn wf_head_tail(&self) -> bool{
        &&&
        self.head.is_Some() ==> (self.head.unwrap() == self.ptrs_seq@[0] && self.perms@[self.head.unwrap()].value().prev.is_None())
        &&&
        self.tail.is_Some() ==> (self.tail.unwrap() == self.ptrs_seq@[self.len - 1] && self.perms@[self.tail.unwrap()].value().next.is_None())
        &&&
        self.len == 1 ==> (self.tail.unwrap() == self.head.unwrap() && self.perms@[self.head.unwrap()].value().next.is_None() && self.perms@[self.tail.unwrap()].value().prev.is_None())
    }

    pub closed spec fn ptrs_unique(&self) -> bool {
        forall|i:int, j:int| 
            #![trigger self.ptrs_seq@[i], self.ptrs_seq@[j]]
            0<=i<self.len() && 0<=j<self.len() && i != j ==> self.ptrs_seq@[i] != self.ptrs_seq@[j]
    }

    pub closed spec fn perms_wf(&self) -> bool{
        &&&
        self.perms@.dom() =~= self.ptrs_seq@.to_set()
        &&&
        forall|n_ptr: DLLNodePointer| 
            #![trigger self.perms@[n_ptr].is_init()]
            #![trigger self.perms@[n_ptr].addr()]
            self.perms@.dom().contains(n_ptr) 
            ==> self.perms@[n_ptr].is_init() && self.perms@[n_ptr].addr() == n_ptr
    }

    pub closed spec fn ptrs_seq_wf(&self) -> bool{
        &&&
        forall|i:int|
            #![trigger self.perms@[self.ptrs_seq@[i]].value().next]
            0 <= i < self.len() - 1 
            ==> 
            self.perms@[self.ptrs_seq@[i]].value().next.is_Some() && self.perms@[self.ptrs_seq@[i]].value().next.unwrap() == self.ptrs_seq@[i + 1]
        &&&
        forall|i:int|
            #![trigger self.perms@[self.ptrs_seq@[i]].value().prev]
            1 <= i < self.len()  
            ==> 
            self.perms@[self.ptrs_seq@[i]].value().prev.is_Some() && self.perms@[self.ptrs_seq@[i]].value().prev.unwrap() == self.ptrs_seq@[i - 1]
    }

    pub closed spec fn value_seq_wf(&self) -> bool{
        &&&
        forall|i:int|
            #![trigger self.perms@[self.ptrs_seq@[i]].value().payload]
            #![trigger self.value_seq@[i]]
            0 <= i < self.len()  
            ==> 
            self.perms@[self.ptrs_seq@[i]].value().payload == self.value_seq@[i]
    }

    pub closed spec fn wf(&self) -> bool{
        &&&
        self.wf_len()
        &&&
        self.wf_head_tail()
        &&&
        self.ptrs_unique()
        &&&
        self.perms_wf()
        &&&
        self.ptrs_seq_wf()
        &&&
        self.value_seq_wf()
    }

    fn push_empty(&mut self, ptr:DLLNodePointer, perm: Tracked<PointsTo<DLLNode<V>>>) 
        requires
            old(self).wf(),
            old(self).len() == 0,
            perm@.addr() == ptr,
            perm@.is_init(),
        ensures
            self.wf(),
            self.len() == old(self).len() + 1,
            self@ =~= old(self)@.push(perm@.value().payload),
            self.closure() =~= old(self).closure().insert(ptr),
            forall|n_ptr: DLLNodePointer| 
                #![trigger self.resolve(n_ptr)]
                old(self).closure().contains(n_ptr) 
                ==>
                self.resolve(n_ptr) == old(self).resolve(n_ptr),
            self.resolve(ptr) == perm@.value().payload,
    {
        let Tracked(mut perm) = perm;
        set_prev::<V>(ptr, Tracked(&mut perm), None);
        set_next::<V>(ptr, Tracked(&mut perm), None);
        self.len = 1;
        self.head = Some(ptr);
        self.tail = Some(ptr);

        proof{
            self.ptrs_seq@ = self.ptrs_seq@.push(ptr);
            self.value_seq@ = self.value_seq@.push(perm.value().payload);
            self.perms.borrow_mut().tracked_insert(ptr, perm);
        }
        assert(self.wf());
    }

    fn push_non_empty_tail(&mut self, ptr:DLLNodePointer, perm: Tracked<PointsTo<DLLNode<V>>>) 
        requires
            old(self).wf(),
            old(self).len() != 0,
            old(self).len() != usize::MAX,
            perm@.addr() == ptr,
            perm@.is_init(),
            old(self).closure().contains(ptr) == false,
        ensures
            self.wf(),
            self.len() == old(self).len() + 1,
            self@ =~= old(self)@.push(perm@.value().payload),
            self.closure() =~= old(self).closure().insert(ptr),
            forall|n_ptr: DLLNodePointer| 
                #![trigger self.resolve(n_ptr)]
                old(self).closure().contains(n_ptr) 
                ==>
                self.resolve(n_ptr) == old(self).resolve(n_ptr),
            self.resolve(ptr) == perm@.value().payload,
    {
        let Tracked(mut perm) = perm;
        assert(self.tail.is_Some());
        let old_tail_ptr = self.tail.unwrap();
        self.tail = Some(ptr);
        self.len = self.len + 1;
        set_prev::<V>(ptr, Tracked(&mut perm), Some(old_tail_ptr));
        set_next::<V>(ptr, Tracked(&mut perm), None);
        
        let tracked mut old_tail_perm = self.perms.borrow_mut().tracked_remove(old_tail_ptr);
        set_next::<V>(old_tail_ptr, Tracked(&mut old_tail_perm), Some(ptr));
        proof{
            self.ptrs_seq@ = self.ptrs_seq@.push(ptr);
            self.value_seq@ = self.value_seq@.push(perm.value().payload);
            self.perms.borrow_mut().tracked_insert(ptr, perm);
            self.perms.borrow_mut().tracked_insert(old_tail_ptr, old_tail_perm);
        }
        assert(self.ptrs_seq@.to_set() =~= old(self).ptrs_seq@.to_set().insert(ptr)) by {
            seq_push_unique_to_set_lemma::<DLLNodePointer>();
        };
        assert(self.wf_len());
        assert(self.wf_head_tail());
        assert(self.ptrs_unique());
        assert(self.perms_wf());
        assert(self.ptrs_seq_wf());
        assert(self.value_seq_wf());
        assert(self.wf());

    }

    pub fn push_back(&mut self, ptr:DLLNodePointer, perm: Tracked<PointsTo<DLLNode<V>>>) 
    requires
        old(self).wf(),
        perm@.addr() == ptr,
        perm@.is_init(),
        old(self).len() != usize::MAX,
        old(self).closure().contains(ptr) == false,
    ensures
        self.wf(),
        self.len() == old(self).len() + 1,
        self@ =~= old(self)@.push(perm@.value().payload),
        self.closure() =~= old(self).closure().insert(ptr),
        forall|n_ptr: DLLNodePointer| 
            #![trigger self.resolve(n_ptr)]
            old(self).closure().contains(n_ptr) 
            ==>
            self.resolve(n_ptr) == old(self).resolve(n_ptr),
        self.resolve(ptr) == perm@.value().payload,
    {
        if self.len() == 0{
            self.push_empty(ptr, perm);
        }else{
            self.push_non_empty_tail(ptr, perm);
        }
    }

    fn pop_head_empty(&mut self) -> (ret:(DLLNodePointer, Tracked<PointsTo<DLLNode<V>>>))
        requires
            old(self).wf(),
            old(self).len() == 1,
        ensures
            self.wf(),
            self.len() == old(self).len() - 1,
            self@ =~= old(self)@.drop_first(),
            self.closure() =~= old(self).closure().remove(ret.0),
            self.closure() =~= Set::empty(),
            ret.1@.addr() == ret.0,
            ret.1@.is_init(),
            ret.1@.value().payload == old(self)@[0],
    {
        let old_head_ptr = self.head.unwrap();
        self.head = None;
        self.tail = None;
        self.len = 0;
        proof{
            self.ptrs_seq@ = self.ptrs_seq@.drop_first();
            self.value_seq@ = self.value_seq@.drop_first();
        }
        let tracked mut old_head_perm = self.perms.borrow_mut().tracked_remove(old_head_ptr);
        assert(self.wf());
        return (old_head_ptr, Tracked(old_head_perm));
    }

    fn pop_head_non_empty(&mut self) -> (ret:(DLLNodePointer, Tracked<PointsTo<DLLNode<V>>>))
        requires
            old(self).wf(),
            old(self).len() > 1,
        ensures
            self.wf(),
            self.len() == old(self).len() - 1,
            self@ =~= old(self)@.drop_first(),
            self.closure() =~= old(self).closure().remove(ret.0),
            ret.1@.addr() == ret.0,
            ret.1@.is_init(),
            ret.1@.value().payload == old(self)@[0],
            forall|n_ptr: DLLNodePointer| 
            #![trigger self.resolve(n_ptr)]
                self.closure().contains(n_ptr) 
                ==>
                self.resolve(n_ptr) == old(self).resolve(n_ptr),
    {
        proof{
            seq_drop_unique_to_set_lemma::<DLLNodePointer>();
        }
        let old_head_ptr = self.head.unwrap();
        
        let tracked mut old_head_perm = self.perms.borrow_mut().tracked_remove(old_head_ptr);
        let old_head_node = PPtr::<DLLNode<V>>::from_usize(old_head_ptr).borrow(Tracked(&old_head_perm));
        let old_head_next_ptr = old_head_node.next.unwrap();
        self.head = Some(old_head_next_ptr);
        self.len = self.len - 1;
        let tracked mut new_head_perm = self.perms.borrow_mut().tracked_remove(old_head_next_ptr);
        set_prev::<V>(old_head_next_ptr, Tracked(&mut new_head_perm), None);
        proof{
            self.perms.borrow_mut().tracked_insert(old_head_next_ptr, new_head_perm);
            self.ptrs_seq@ = self.ptrs_seq@.drop_first();
            self.value_seq@ = self.value_seq@.drop_first();
        }
        assert(self.wf_len());
        assert(self.wf_head_tail());
        assert(self.ptrs_unique());
        assert(self.perms@.dom() =~= self.ptrs_seq@.to_set());
        assert(forall|n_ptr: DLLNodePointer| 
            #![trigger self.perms@[n_ptr].is_init()]
            #![trigger self.perms@[n_ptr].addr()]
            self.perms@.dom().contains(n_ptr) 
            ==> self.perms@[n_ptr].is_init() && self.perms@[n_ptr].addr() == n_ptr);
        assert(self.perms_wf());
        assert(self.ptrs_seq_wf());
        assert(self.value_seq_wf());
        assert(self.wf());
        return (old_head_ptr, Tracked(old_head_perm));
    }

    pub fn pop_head(&mut self) -> (ret:(DLLNodePointer, Tracked<PointsTo<DLLNode<V>>>))
        requires
            old(self).wf(),
            old(self).len() != 0,
        ensures
            self.wf(),
            self.len() == old(self).len() - 1,
            self@ =~= old(self)@.drop_first(),
            self.closure() =~= old(self).closure().remove(ret.0),
            ret.1@.addr() == ret.0,
            ret.1@.is_init(),
            ret.1@.value().payload == old(self)@[0],
            forall|n_ptr: DLLNodePointer| 
            #![trigger self.resolve(n_ptr)]
                self.closure().contains(n_ptr) 
                ==>
                self.resolve(n_ptr) == old(self).resolve(n_ptr),
    {
        if self.len() == 1{
            return self.pop_head_empty();
        }else{
            return self.pop_head_non_empty();
        }
    }
}

}