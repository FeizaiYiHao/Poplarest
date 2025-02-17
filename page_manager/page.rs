use vstd::prelude::*;
verus! {
    use crate::define::*;
    use vstd::simple_pptr::PPtr;
    use vstd::simple_pptr::PointsTo;
    
    pub struct Page{
        pub meta_data: PageMetaData,
        page_linkedlist_metadata: PageLinkedlistMetaData,
    }

    pub struct PageMetaData{
        pub addr: PagePtr,
        pub state: PageState,
        pub is_io_page: bool,
        pub ref_count: usize,
        pub owning_container: Option<ContainerPtr>,

        // pub mappings: Ghost<Set<(Pcid,VAddr)>>,
        // pub io_mappings: Ghost<Set<(IOid,VAddr)>>,

        
        pub page_linkedlist_metadata_perm: Tracked<Option<PointsTo<PageLinkedlistMetaData>>>,
    }

    #[derive(Clone, Copy)]
    pub struct PageLinkedlistMetaData{
        pub addr: PagePtr,
        pub prev: Option<PPtr<PageLinkedlistMetaData>>,
        pub next: Option<PPtr<PageLinkedlistMetaData>>,
    }
}