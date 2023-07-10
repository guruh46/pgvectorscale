//! This module contains ports of Postgres static functions and #defines not in pgrx.
//! Following pgrx conventions, we keep function names as close to Postgres as possible.
//! Thus, we don't follow rust naming conventions.

use memoffset::*;
use pgrx::pg_sys::Pointer;

/// Given a valid Page pointer, return address of the "Special Pointer" (custom info at end of page)
///
/// # Safety
///
/// This function cannot determine if the `page` argument is really a non-null pointer to a [`Page`].
#[inline(always)]
#[allow(non_snake_case)]
pub unsafe fn PageGetSpecialPointer(page: pgrx::pg_sys::Page) -> Pointer {
    // PageValidateSpecialPointer(page);
    // return (char *) page + ((PageHeader) page)->pd_special;
    PageValidateSpecialPointer(page);
    let header = page.cast::<pgrx::pg_sys::PageHeaderData>();
    page.cast::<std::os::raw::c_char>()
        .add((*header).pd_special as usize)
}

#[allow(non_snake_case)]
pub unsafe fn PageValidateSpecialPointer(page: pgrx::pg_sys::Page) {
    //Assert(page);
    //Assert(((PageHeader) page)->pd_special <= BLCKSZ);
    //Assert(((PageHeader) page)->pd_special >= SizeOfPageHeaderData);
    assert!(!page.is_null());
    let header = page.cast::<pgrx::pg_sys::PageHeaderData>();
    assert!((*header).pd_special <= pgrx::pg_sys::BLCKSZ as u16);
    assert!((*header).pd_special >= SizeOfPageHeaderData as u16);
}

#[allow(non_upper_case_globals)]
const SizeOfPageHeaderData: usize = offset_of!(pgrx::pg_sys::PageHeaderData, pd_linp);

#[allow(non_snake_case)]
pub unsafe fn PageGetContents(page: pgrx::pg_sys::Page) -> *mut std::os::raw::c_char {
    //return (char *) page + MAXALIGN(SizeOfPageHeaderData);
    page.cast::<std::os::raw::c_char>()
        .add(pgrx::pg_sys::MAXALIGN(SizeOfPageHeaderData))
}