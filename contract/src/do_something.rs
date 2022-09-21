use crate::util::*;

#[export_name="Method_2"]
pub fn do_something(_params: *const usize) {
    shared()
}