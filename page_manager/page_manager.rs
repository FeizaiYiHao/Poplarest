use vstd::prelude::*;
verus! {
    use crate::define::*;
    use crate::page_manager::page::*;
    use crate::array::Array;
    use crate::rwlock::*;
    use crate::lock_agent::*;

    pub struct PageManager{
        pub page_array: Array<Page, NUM_PAGES>,

        pub container_page_4k_dict: Ghost<Map<ContainerPtr, Set<PageID>>>,
        pub container_page_2m_dict: Ghost<Map<ContainerPtr, Set<PageID>>>,
        pub container_page_1g_dict: Ghost<Map<ContainerPtr, Set<PageID>>>,
        pub mapped_pages_4k: Ghost<Set<PageID>>,
        pub mapped_pages_2m: Ghost<Set<PageID>>,
        pub mapped_pages_1g: Ghost<Set<PageID>>,
    }

    impl PageManager{
        pub open spec fn page_array_wf(&self) -> bool{
            &&&
            self.page_array.wf()
        }

        pub open spec fn mapped_pages_4k_wf(&self) -> bool{
            &&&
            forall|p_id:PageID| 
                #![auto] 
                self.mapped_pages_4k@.contains(p_id) ==>
                    self.page_array@[p_id as int].write_locked() || 
                    (self.page_array@[p_id as int]@.state == PageState::Mapped &&
                        self.page_array@[p_id as int]@.size == PageSize::SZ4k)
            &&&
            forall|p_id:PageID| 
                #![auto] 
                !self.page_array@[p_id as int].write_locked() && self.page_array@[p_id as int]@.state == PageState::Mapped && 
                    self.page_array@[p_id as int]@.size == PageSize::SZ4k ==>
                    self.mapped_pages_4k@.contains(p_id)
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
                self.page_array@[page_id as int].unchanged() == old(self).page_array@[page_id as int].unchanged(),
                self.page_array@[page_id as int].lock_id() == old(self).page_array@[page_id as int].lock_id(),
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
                self.page_array@[page_id as int].unchanged() == old(self).page_array@[page_id as int].unchanged(),
                self.page_array@[page_id as int].lock_id() == old(self).page_array@[page_id as int].lock_id(),
                self.page_array@[page_id as int]@ =~= old(self).page_array@[page_id as int]@,
                forall|p_id:PageID| #![auto] 0 <= p_id < NUM_PAGES && p_id != page_id ==> self.page_array@[p_id as int] == old(self).page_array@[p_id as int],
        {
            //TODO
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
                    self.page_array@[page_id as int].unchanged() == false && 
                    self.page_array@[page_id as int].lock_id() == old(self).page_array@[page_id as int].lock_id() &&
                    self.page_array@[page_id as int]@ =~= old(self).page_array@[page_id as int]@ &&
                    forall|p_id:PageID| #![auto] 0 <= p_id < NUM_PAGES && p_id != page_id ==> self.page_array@[p_id as int] == old(self).page_array@[p_id as int]
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

}