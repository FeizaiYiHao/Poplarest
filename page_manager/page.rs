use vstd::prelude::*;
use core::sync::atomic::AtomicBool;

verus! {
    use crate::define::*;
    use vstd::simple_pptr::PPtr;
    use vstd::simple_pptr::PointsTo;
    use crate::lock_agent::*;
    use crate::rwlock::*;
    
    #[derive(Clone, Copy)]
    pub struct PageLinkedlistMetaData{
        pub addr: PagePtr,
        pub prev: Option<PPtr<PageLinkedlistMetaData>>,
        pub next: Option<PPtr<PageLinkedlistMetaData>>,
    }

    impl PageLinkedlistMetaData{
        pub fn new() -> Self{
            Self{
                addr:0,
                prev:None,
                next:None,
            }
        }
    }

    pub struct PageView{
        pub addr: PagePtr,
        pub state: PageState,
        pub is_io_page: bool,
        pub ref_count: usize,
        pub owning_container: Option<ContainerPtr>,
    }

    pub struct Page{
        // built-in lock, only used when page is in "mapped" state
        locked: AtomicBool,
        num_writer:usize,
        num_readers: usize,
    
        writing_thread: Ghost<Option<ThreadID>>,
        reading_threads: Ghost<Set<ThreadID>>,

        //metadata
        addr: PagePtr,
        state: PageState,
        is_io_page: bool,
        ref_count: usize,
        owning_container: Option<ContainerPtr>,

        // reference counters
        mappings: Ghost<Seq<Set<VAddr>>>,
        io_mappings: Ghost<Seq<Set<VAddr>>>,

        //per-container linkedlist node perm
        page_linkedlist_metadata: PageLinkedlistMetaData,
        page_linkedlist_metadata_perm: Tracked<Option<PointsTo<PageLinkedlistMetaData>>>
    }

    impl Page{

        pub spec fn lock_id(self) -> int;

        pub spec fn unchanged(self) -> bool;
    
        pub open spec fn lock_id_pair(self) -> LockIDPair{
            (self.lock_major(), self.lock_minor())
        }
    
        pub open spec fn lock_major(self) -> LockMajorID{
            PageLockMajor
        }
    
        pub closed spec fn lock_minor(self) -> LockMinorID{
            self.addr
        }

        #[verifier(external_body)]
        pub fn new() -> Self{
            Self{
                locked: AtomicBool::new(false),
                num_writer: 0,
                num_readers: 0,
            
                writing_thread: Ghost(None),
                reading_threads: Ghost(Set::empty()),
        
                addr: 0,
                state: PageState::Allocated,
                is_io_page: false,
                ref_count: 0,
                owning_container: None,
        
                // mappings: Ghost(Set::empty()),
                // io_mappings: Ghost(Set::empty()),
        
                page_linkedlist_metadata: PageLinkedlistMetaData::new(),
                page_linkedlist_metadata_perm: Tracked::assume_new(),
            }
        }

        pub closed spec fn addr(&self) -> PagePtr{
            self.addr
        }
        pub closed spec fn state(&self) -> PageState{
            self.state
        }
        pub closed spec fn is_io_page(&self) -> bool{
            self.is_io_page
        }
        pub closed spec fn ref_count(&self) -> usize{
            self.ref_count
        }
        pub closed spec fn total_map_

        pub closed spec fn owning_container(&self) -> Option<ContainerPtr>{
            self.owning_container
        }

        pub open spec fn view(&self) -> PageView{
            PageView{
                addr: self.addr(),
                state: self.state(),
                is_io_page: self.is_io_page(),
                ref_count: self.ref_count(),
                owning_container: self.owning_container(),
            }
        }

        pub closed spec fn writing_thread(&self) -> Option<ThreadID>{
            self.writing_thread@
        }
        pub closed spec fn reading_threads(&self) -> Set<ThreadID>{
            self.reading_threads@
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
                step_lock_aquire_ensures(old(lock_agent), lock_agent, old(self).lock_id_pair()),
                old(self).lock_id_pair() =~= self.lock_id_pair(),
                self.unchanged() == old(self).unchanged(),
                self.lock_id() == old(self).lock_id(),
                ret@.lock_id() == self.lock_id(),
                self@ =~= old(self)@,
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
                step_lock_release_ensures(old(lock_agent), lock_agent, old(self).lock_id_pair()),
                old(self).lock_id_pair() =~= self.lock_id_pair(),
                self.unchanged() == old(self).unchanged(),
                self.lock_id() == old(self).lock_id(),
                self@ =~= old(self)@,
        {
        }

        pub fn read(&self, Tracked(read_perm):Tracked<&ReadPerm>) -> (ret:PageView)
            requires
                read_perm.lock_id() == self.lock_id(),
            ensures
                ret == self@,
        {
            PageView{
                addr: self.addr,
                state: self.state,
                is_io_page: self.is_io_page,
                ref_count: self.ref_count,
                owning_container: self.owning_container,
            }
        }


    }
}