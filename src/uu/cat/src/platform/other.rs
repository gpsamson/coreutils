// For non-Unix, non-Windows platforms (e.g., WASI), provide a simple
// implementation that conservatively allows operations by returning false.

use std::os::fd::AsFd;

#[inline]
pub fn is_unsafe_overwrite<I: AsFd, O: AsFd>(_input: &I, _output: &O) -> bool {
    // On WASI and other platforms, we don't have inode/device comparisons,
    // so we conservatively return false to allow the operation.
    // This may allow some unsafe overwrites, but WASI environments are
    // typically sandboxed anyway.
    false
}
