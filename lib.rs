use vstd::prelude::verus;

pub mod define;
pub mod thread;
pub mod process;
pub mod process_manager;
pub mod slinkedlist;
pub mod lemma;
pub mod util;
pub mod array;
pub mod proc_lock;
pub mod thread_gurad;
verus! {

global size_of usize == 8;

}

fn main(){

}