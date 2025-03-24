use vstd::prelude::*;
use core::sync::atomic::AtomicBool;

verus! {
    use crate::define::*;
    use vstd::simple_pptr::PPtr;
    use vstd::simple_pptr::PointsTo;
    // use crate::lock_agent::*;
    use crate::rwlock::*;

    #[derive(Clone, Copy)]
    pub struct PageLinkedlistMetaData{
        pub addr: PagePtr,
        pub id: PageID,
        pub prev: Option<PPtr<PageLinkedlistMetaData>>,
        pub next: Option<PPtr<PageLinkedlistMetaData>>,
    }

    impl PageLinkedlistMetaData{
        pub fn new() -> Self{
            Self{
                addr:0,
                id: 0,
                prev:None,
                next:None,
            }
        }
    }

    pub struct PageView{
        pub addr: PagePtr,
        pub state: PageState,
        pub size: PageSize,
        pub is_io_page: bool,
        pub ref_count: usize,
        pub owning_container: Option<ContainerPtr>,
        pub page_size: PageSize,
    }

    // mapped pages can be read/write locked by anyone
    // merged/allocated pages can be read locked by anyone, but writed locked only by the owning container
    // 
    pub struct Page{
        locked: AtomicBool,
        num_writer:bool,
        num_readers: usize,
        lock_state:LockState,
    
        writing_thread: Ghost<Option<ThreadID>>,
        reading_threads: Ghost<Set<ThreadID>>,

        //metadata
        addr: PagePtr,
        state: PageState,
        size: PageSize,
        is_io_page: bool,
        ref_count: usize,
        owning_container: Option<ContainerPtr>,
        page_size: PageSize,
        rev_ptr: DLLNodePointer,

        // reference counters
        mappings: Ghost<Set<(Pcid, VAddr)>>,
        io_mappings: Ghost<Set<(IOid,VAddr)>>,

        //per-container linkedlist node perm
        page_linkedlist_metadata: PageLinkedlistMetaData,
        page_linkedlist_metadata_perm: Tracked<Option<PointsTo<PageLinkedlistMetaData>>>,
    }

    impl RWLock for Page{
        spec fn lock_id(&self) -> int;
    
        open spec fn lock_id_pair(&self) -> LockIDPair{
            (self.lock_major(), self.lock_minor())
        }
    
        spec fn separate(&self) -> bool;

        open spec fn lock_major(&self) -> LockMajorID{
            PageLockMajor
        }
    
        closed spec fn lock_minor(&self) -> LockMinorID{
            self.addr
        }

        closed spec fn writing_thread(&self) -> Option<ThreadID>{
            self.writing_thread@
        }

        closed spec fn reading_threads(&self) -> Set<ThreadID>{
            self.reading_threads@
        }

        closed spec fn write_locked(&self) -> bool {
            self.num_writer
        }

        closed spec fn read_locked(&self) -> bool {
            self.num_readers != 0
        }
    }

    impl Page{

        pub closed spec fn lock_wf(&self) -> bool{
            &&&
            self.num_writer == self.writing_thread@.is_some()
            &&&
            self.num_readers == self.reading_threads@.len()
        }

        #[verifier(external_body)]
        pub fn new() -> Self{
            Self{
                locked: AtomicBool::new(false),
                num_writer: false,
                num_readers: 0,
                lock_state: LockState::Unlocked,
            
                writing_thread: Ghost(None),
                reading_threads: Ghost(Set::empty()),
        
                addr: 0,
                state: PageState::Allocated,
                size: PageSize::SZ4k,
                is_io_page: false,
                ref_count: 0,
                owning_container: None,
                page_size:PageSize::SZ4k,
                rev_ptr: 0,
        
                mappings: Ghost(Set::empty()),
                io_mappings: Ghost(Set::empty()),
        
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
        pub closed spec fn size(&self) -> PageSize{
            self.size
        }
        pub closed spec fn is_io_page(&self) -> bool{
            self.is_io_page
        }
        pub closed spec fn ref_count(&self) -> usize{
            self.ref_count
        }
        pub closed spec fn page_size(&self) -> PageSize{
            self.page_size
        }
        pub closed spec fn mappings(&self) -> Set<(Pcid, VAddr)>{
            self.mappings@
        }
        pub closed spec fn io_mappings(&self) -> Set<(IOid, VAddr)>{
            self.io_mappings@
        }
        pub closed spec fn owning_container(&self) -> Option<ContainerPtr>{
            self.owning_container
        }

        pub open spec fn view(&self) -> PageView{
            PageView{
                addr: self.addr(),
                state: self.state(),
                size: self.size(),
                is_io_page: self.is_io_page(),
                ref_count: self.ref_count(),
                owning_container: self.owning_container(),
                page_size: self.page_size()
            }
        }

        // pub closed spec fn writing_thread(&self) -> Option<ThreadID>{
        //     self.writing_thread@
        // }
        // pub closed spec fn reading_threads(&self) -> Set<ThreadID>{
        //     self.reading_threads@
        // }



        pub closed spec fn linkedlist_metadata_perm_wf(&self) -> bool{
            &&&
            self.state != PageState::Allocated <==> self.page_linkedlist_metadata_perm@.is_some() && self.page_linkedlist_metadata_perm@.unwrap().is_init()
        }
        pub closed spec fn reference_counting_wf(&self) -> bool{
            &&&
            self.state == PageState::Mapped ==> 
                self.mappings@.len() + self.io_mappings@.len() == self@.ref_count
        }
        pub closed spec fn page_size_wf(&self) -> bool {
            &&&
            self.state == PageState::Unavailable <==> self.page_size == PageSize::Unavailable
        }
        pub closed spec fn wf(&self) -> bool {
            &&&
            self.linkedlist_metadata_perm_wf()
            &&&
            self.reference_counting_wf()
            &&&
            self.page_size_wf()
        }


        // #[verifier(external_body)]
        // pub fn read_lock_mapped(&mut self, Tracked(lock_agent): Tracked<&mut LockAgent>) -> (ret:Tracked<ReadPerm>)
        //     requires
        //         old(self).reading_threads().contains(old(lock_agent).thread_id) == false,
        //         old(self).writing_thread().is_None() || old(self).writing_thread().unwrap() != old(lock_agent).thread_id,
        //         step_lock_aquire_requires(old(lock_agent), old(self).lock_id_pair()),
        //         old(self)@.state == PageState::Mapped,
        //     ensures
        //         self.reading_threads() =~= old(self).reading_threads().insert(lock_agent.thread_id),
        //         old(self).writing_thread().is_None(),
        //         self.writing_thread() =~= old(self).writing_thread(),
        //         step_lock_aquire_ensures(old(lock_agent), lock_agent, old(self).lock_id_pair()),
        //         old(self).lock_id_pair() =~= self.lock_id_pair(),
        //         self.separate() == old(self).separate(),
        //         self.lock_id() == old(self).lock_id(),
        //         ret@.lock_id() == self.lock_id(),
        //         self@ =~= old(self)@,
        // {
        //     //TODO
        //     Tracked::assume_new()
        // }

        // #[verifier(external_body)]
        // pub fn read_unlock_mapped(&mut self, Tracked(lock_agent): Tracked<&mut LockAgent>, Tracked(read_perm):Tracked<ReadPerm>)
        //     requires
        //         old(self).reading_threads().contains(old(lock_agent).thread_id),
        //         step_lock_release_requires(old(lock_agent), old(self).lock_id_pair()),
        //         read_perm.lock_id() == old(self).lock_id(),
        //         old(self)@.state == PageState::Mapped,
        //     ensures
        //         self.reading_threads() =~= old(self).reading_threads().remove(lock_agent.thread_id),
        //         self.writing_thread() =~= old(self).writing_thread(),
        //         step_lock_release_ensures(old(lock_agent), lock_agent, old(self).lock_id_pair()),
        //         old(self).lock_id_pair() =~= self.lock_id_pair(),
        //         self.separate() == old(self).separate(),
        //         self.lock_id() == old(self).lock_id(),
        //         self@ =~= old(self)@,
        // {
        //     //TODO
        // }

        // #[verifier(external_body)]
        // pub fn read_lock_mapped(&mut self, Tracked(lock_agent): Tracked<&mut LockAgent>) -> (ret:Tracked<ReadPerm>)
        //     requires
        //         old(self).reading_threads().contains(old(lock_agent).thread_id) == false,
        //         old(self).writing_thread().is_None() || old(self).writing_thread().unwrap() != old(lock_agent).thread_id,
        //         step_lock_aquire_requires(old(lock_agent), old(self).lock_id_pair()),
        //         old(self)@.state == PageState::Mapped,
        //     ensures
        //         self.reading_threads() =~= old(self).reading_threads().insert(lock_agent.thread_id),
        //         old(self).writing_thread().is_None(),
        //         self.writing_thread() =~= old(self).writing_thread(),
        //         step_lock_aquire_ensures(old(lock_agent), lock_agent, old(self).lock_id_pair()),
        //         old(self).lock_id_pair() =~= self.lock_id_pair(),
        //         self.separate() == old(self).separate(),
        //         self.lock_id() == old(self).lock_id(),
        //         ret@.lock_id() == self.lock_id(),
        //         self@ =~= old(self)@,
        // {
        //     //TODO
        //     Tracked::assume_new()
        // }

        pub fn read(&self, Tracked(read_perm):Tracked<&ReadPerm>) -> (ret:PageView)
            requires
                read_perm.lock_id() == self.lock_id(),
            ensures
                ret == self@,
        {
            PageView{
                addr: self.addr,
                state: self.state,
                size: self.size,
                is_io_page: self.is_io_page,
                ref_count: self.ref_count,
                owning_container: self.owning_container,
                page_size: self.page_size,
            }
        }

    }
}