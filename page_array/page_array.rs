use vstd::prelude::*;
verus! {
    use crate::define::*;
    use crate::page_array::page::*;
    use crate::array::Array;
    use crate::rwlock::*;
    use crate::lock_agent::*;

    pub struct PageArray{
        pub page_array: Array<Page, NUM_PAGES>,
    }

    impl PageArray{
        pub open spec fn page_array_wf(&self) -> bool{
            &&&
            self.page_array.wf()
        }

        pub open spec fn pages_wf(&self) -> bool{
            &&&
            forall|pg_id:PageID| 
                #![trigger self.page_array@[pg_id as int].write_locked()]
                #![trigger self.page_array@[pg_id as int].wf()]
                0 <= pg_id < NUM_PAGES 
                ==>
                (
                    self.page_array@[pg_id as int].write_locked() 
                    ||
                    self.page_array@[pg_id as int].wf()
                )
        }
        
        #[verifier(external_body)]
        pub fn read_lock(&mut self, page_id:PageID, Tracked(lock_agent): Tracked<&mut LockAgent>) -> (ret:Tracked<ReadPerm>)
            requires
                0 <= page_id < NUM_PAGES,
                old(self).page_array@[page_id as int].reading_threads().contains(old(lock_agent).thread_id) == false,
                old(self).page_array@[page_id as int].writing_thread().is_None(),
                step_lock_aquire_requires(old(lock_agent), old(self).page_array@[page_id as int].lock_id_pair()),
            ensures
                self.page_array@[page_id as int].reading_threads() =~= old(self).page_array@[page_id as int].reading_threads().insert(lock_agent.thread_id),
                self.page_array@[page_id as int].writing_thread() =~= old(self).page_array@[page_id as int].writing_thread(),
                step_lock_aquire_ensures(old(lock_agent), lock_agent, old(self).page_array@[page_id as int].lock_id_pair()),
                old(self).page_array@[page_id as int].lock_id_pair() =~= self.page_array@[page_id as int].lock_id_pair(),
                self.page_array@[page_id as int].separate() == old(self).page_array@[page_id as int].separate(),
                self.page_array@[page_id as int].lock_id() == old(self).page_array@[page_id as int].lock_id(),
                ret@.lock_id() == self.page_array@[page_id as int].lock_id(),
                self.page_array@[page_id as int]@ =~= old(self).page_array@[page_id as int]@,
                forall|p_id:PageID| #![auto] 0 <= p_id < NUM_PAGES && p_id != page_id ==> self.page_array@[p_id as int] == old(self).page_array@[p_id as int],
        {
            //TODO
            Tracked::assume_new()
        }

        #[verifier(external_body)]
        pub fn read_unlock(&mut self, page_id:PageID, Tracked(lock_agent): Tracked<&mut LockAgent>, read_perm:Tracked<ReadPerm>) 
            requires
                0 <= page_id < NUM_PAGES,
                old(self).page_array@[page_id as int].reading_threads().contains(old(lock_agent).thread_id),
                step_lock_release_requires(old(lock_agent), old(self).page_array@[page_id as int].lock_id_pair()),
                read_perm@.lock_id() == old(self).page_array@[page_id as int].lock_id(),
            ensures
                self.page_array@[page_id as int].reading_threads() =~= old(self).page_array@[page_id as int].reading_threads().remove(lock_agent.thread_id),
                self.page_array@[page_id as int].writing_thread() =~= old(self).page_array@[page_id as int].writing_thread(),
                step_lock_release_ensures(old(lock_agent), lock_agent, old(self).page_array@[page_id as int].lock_id_pair()),
                old(self).page_array@[page_id as int].lock_id_pair() =~= self.page_array@[page_id as int].lock_id_pair(),
                self.page_array@[page_id as int].separate() == old(self).page_array@[page_id as int].separate(),
                self.page_array@[page_id as int].lock_id() == old(self).page_array@[page_id as int].lock_id(),
                self.page_array@[page_id as int]@ =~= old(self).page_array@[page_id as int]@,
                forall|p_id:PageID| #![auto] 0 <= p_id < NUM_PAGES && p_id != page_id ==> self.page_array@[p_id as int] == old(self).page_array@[p_id as int],
        {
            //TODO
        }

        #[verifier(external_body)]
        pub fn read_upgrade_to_write_lock(&mut self, page_id:PageID, read_perm: Tracked<ReadPerm>, Tracked(lock_agent): Tracked<&mut LockAgent>) -> (ret:Tracked<WritePerm>)
            requires
                0 <= page_id < NUM_PAGES,
                old(self).page_array@[page_id as int].reading_threads().contains(old(lock_agent).thread_id),
                old(self).page_array@[page_id as int].writing_thread().is_None(),
                step_lock_release_requires(old(lock_agent), old(self).page_array@[page_id as int].lock_id_pair()),
                read_perm@.lock_id() =~= old(self).page_array@[page_id as int].lock_id() 
            ensures
                old(self).page_array@[page_id as int].reading_threads().len() == 0,
                self.page_array@[page_id as int].reading_threads() =~= Set::empty(),
                self.page_array@[page_id as int].writing_thread() =~= Some(old(lock_agent).thread_id),
                old(self).page_array@[page_id as int].lock_id_pair() =~= self.page_array@[page_id as int].lock_id_pair(),
                self.page_array@[page_id as int].separate() == false,
                self.page_array@[page_id as int].lock_id() == old(self).page_array@[page_id as int].lock_id(),
                self.page_array@[page_id as int]@ =~= old(self).page_array@[page_id as int]@,
                forall|p_id:PageID| #![auto] 0 <= p_id < NUM_PAGES && p_id != page_id ==> self.page_array@[p_id as int] == old(self).page_array@[p_id as int],
                ret@.lock_id() == self.page_array@[page_id as int].lock_id(),
        {
            //TODO
            Tracked::assume_new()
        }
    
        #[verifier(external_body)]
        pub fn write_lock(&mut self, page_id:PageID, Tracked(lock_agent): Tracked<&mut LockAgent>) -> (ret:Tracked<WritePerm>)
            requires
                0 <= page_id < NUM_PAGES,
                old(self).page_array@[page_id as int].reading_threads().contains(old(lock_agent).thread_id) == false,
                old(self).page_array@[page_id as int].writing_thread().is_None(),
                step_lock_aquire_requires(old(lock_agent), old(self).page_array@[page_id as int].lock_id_pair()),
            ensures
                old(self).page_array@[page_id as int].reading_threads().len() == 0,
                self.page_array@[page_id as int].reading_threads() =~= old(self).page_array@[page_id as int].reading_threads(),
                self.page_array@[page_id as int].writing_thread() =~= Some(old(lock_agent).thread_id),
                step_lock_aquire_ensures(old(lock_agent), lock_agent, old(self).page_array@[page_id as int].lock_id_pair()),
                old(self).page_array@[page_id as int].lock_id_pair() =~= self.page_array@[page_id as int].lock_id_pair(),
                self.page_array@[page_id as int].separate() == false,
                self.page_array@[page_id as int].lock_id() == old(self).page_array@[page_id as int].lock_id(),
                self.page_array@[page_id as int]@ =~= old(self).page_array@[page_id as int]@,
                forall|p_id:PageID| #![auto] 0 <= p_id < NUM_PAGES && p_id != page_id ==> self.page_array@[p_id as int] == old(self).page_array@[p_id as int],
                ret@.lock_id() == self.page_array@[page_id as int].lock_id(),
        {
            //TODO
            Tracked::assume_new()
        }

        #[verifier(external_body)]
        pub fn write_lock_mapped(&mut self, page_id:PageID, Tracked(lock_agent): Tracked<&mut LockAgent>) -> (ret:Option<Tracked<WritePerm>>)
            requires
                0 <= page_id < NUM_PAGES,
                old(self).page_array@[page_id as int].reading_threads().contains(old(lock_agent).thread_id) == false,
                old(self).page_array@[page_id as int].writing_thread().is_None(),
                step_lock_aquire_requires(old(lock_agent), old(self).page_array@[page_id as int].lock_id_pair()),
            ensures
                old(self).page_array@[page_id as int]@.state == PageState::Mapped <==> ret.is_Some(),
                ret.is_Some() ==> 
                (
                    old(self).page_array@[page_id as int].reading_threads().len() == 0 &&
                    self.page_array@[page_id as int].reading_threads() =~= old(self).page_array@[page_id as int].reading_threads() &&
                    self.page_array@[page_id as int].writing_thread() =~= Some(old(lock_agent).thread_id) &&
                    step_lock_aquire_ensures(old(lock_agent), lock_agent, old(self).page_array@[page_id as int].lock_id_pair()) &&
                    old(self).page_array@[page_id as int].lock_id_pair() =~= self.page_array@[page_id as int].lock_id_pair() &&
                    self.page_array@[page_id as int].separate() == false && 
                    self.page_array@[page_id as int].lock_id() == old(self).page_array@[page_id as int].lock_id() &&
                    self.page_array@[page_id as int]@ =~= old(self).page_array@[page_id as int]@ &&
                    forall|p_id:PageID| #![auto] 0 <= p_id < NUM_PAGES && p_id != page_id ==> self.page_array@[p_id as int] == old(self).page_array@[p_id as int] &&
                    ret.unwrap()@.lock_id() == self.page_array@[page_id as int].lock_id()
                ),
                ret.is_None() ==>
                (
                    self.page_array =~= old(self).page_array
                ),
        {
            //TODO
            Some(Tracked::assume_new())
        }

        #[verifier(external_body)]
        pub fn write_unlock(&mut self, page_id:PageID, Tracked(lock_agent): Tracked<&mut LockAgent>, write_perm:Tracked<WritePerm>)
            requires
                0 <= page_id < NUM_PAGES,
                old(self).page_array@[page_id as int].writing_thread().unwrap() == old(lock_agent).thread_id,
                step_lock_release_requires(old(lock_agent), old(self).page_array@[page_id as int].lock_id_pair()),
                write_perm@.lock_id() == old(self).page_array@[page_id as int].lock_id(),
            ensures
                self.page_array@[page_id as int].reading_threads() =~= old(self).page_array@[page_id as int].reading_threads(),
                self.page_array@[page_id as int].writing_thread().is_None(),
                step_lock_release_ensures(old(lock_agent), lock_agent, old(self).page_array@[page_id as int].lock_id_pair()),
                old(self).page_array@[page_id as int].lock_id_pair() =~= self.page_array@[page_id as int].lock_id_pair(),
                self.page_array@[page_id as int].lock_id() == old(self).page_array@[page_id as int].lock_id(),
                self.page_array@[page_id as int] =~= old(self).page_array@[page_id as int],
                forall|p_id:PageID| #![auto] 0 <= p_id < NUM_PAGES && p_id != page_id ==> self.page_array@[p_id as int] == old(self).page_array@[p_id as int]
        {
            //TODO
        }
}

    pub ghost struct PageGhost{
        pub mappings_4k: Map<PagetableID, Set<VAddr>>,
        pub writing_thread: Option<ThreadID>,
    }

    pub open spec fn write_lock_aquires(old: PageArrayGhost, page_id:PageID) -> bool
    {
        &&&
        old.page_array[page_id as int].writing_thread.is_None()
    }

    pub open spec fn write_lock_ensures(old: PageArrayGhost, new: PageArrayGhost, page_id:PageID, thread_id:ThreadID) -> bool
    {
        &&&
        new.page_array[page_id as int].writing_thread == Some(thread_id)
        &&&
        old.page_array.len() =~= new.page_array.len()
        &&&
        forall|p_id:PageID| #![auto] 0 <= p_id < NUM_PAGES && p_id != page_id ==> 
            old.page_array[p_id as int] == new.page_array[p_id as int]
        &&&
        old.page_array[page_id as int].mappings_4k == new.page_array[page_id as int].mappings_4k
    }

    pub ghost struct PageArrayGhost{
        pub page_array: Seq<PageGhost>,
    }

    impl PageArrayGhost{
        #[verifier(external_body)]
        pub fn write_lock(&mut self, page_id:PageID, thread_id:ThreadID)
            requires 
                write_lock_aquires(*old(self), page_id),
            ensures
                write_lock_ensures(*old(self), *self, page_id, thread_id),
        {}
    }

}