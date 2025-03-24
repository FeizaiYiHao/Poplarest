use vstd::prelude::verus;

pub mod define;
pub mod page_array;
pub mod pagetable_manager;
pub mod rwlock;
pub mod lock_agent;
pub mod state_machine;
pub mod doubly_linked_list;
// pub mod thread;
pub mod process_manager;
// pub mod process_manager;
// pub mod slinkedlist;
pub mod lemma;
pub mod util;
pub mod array;
// pub mod proc_lock;
// pub mod thread_gurad;
verus! {

global size_of usize == 8;

}

fn main(){

}