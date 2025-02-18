use vstd::prelude::*;

verus! {
    pub ghost struct ShradLockMap{
        map: Map<ProcID, Set<Vaddr>>,
        lock: Map<ProcID, >
    }
}