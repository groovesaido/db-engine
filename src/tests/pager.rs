/*                                     INTEGRATION TESTING FOR PAGER              */
use crate::pager::{Page, Pager, memory::MemoryPager};
#[test]
fn allocate_and_read_page() {
    let mut pager = MemoryPager::new();

    // Verifying that consecutive allocations return distinct page ids
    let p1 = pager.allocate_page().unwrap();
    let p2 = pager.allocate_page().unwrap();
    assert_ne!(p1, p2);

    // Prepare a page with test data
    let mut page = Page::zeroed();
    page.as_slice_mut()[0] = 56;
    page.write_u32(100, 0xDEADBEEF);

    // Writing to p1 and reading it back
    pager.write_page(p1, &page).unwrap();
    let read_p1 = pager.read_page(p1).unwrap();

    assert_eq!(read_p1.as_slice()[0], 56);
    assert_eq!(read_p1.read_u32(100), 0xDEADBEEF);

    // Ensure p2 remains isolated and untouched (zeroed)
    let read_p2 = pager.read_page(p2).unwrap();
    assert_eq!(read_p2.as_slice()[0], 0);
    assert_eq!(read_p2.read_u32(100), 0);
}

#[test]
fn allocated_page_is_zeroed() {
    let mut pager = MemoryPager::new();
    let p = pager.allocate_page().unwrap();
    let page = pager.read_page(p).unwrap();
    assert!(page.as_slice().iter().all(|b| b == &0));
}

#[test]
fn freed_page_is_reused() {
    let mut pager = MemoryPager::new();
    let a = pager.allocate_page().unwrap();
    let b = pager.allocate_page().unwrap();
    pager.free_page(a).unwrap();
    let c = pager.allocate_page().unwrap();
    assert_eq!(c, a, "resuse freed page");
    assert_eq!(pager.page_count(), 2, "ensure no new page is created");
    let _ = b;
}

#[test]
fn read_out_of_range_errors() {
    let pager = MemoryPager::new();
    assert!(pager.read_page(1).is_err());
    assert!(pager.read_page(0).is_err());
}

#[test]
fn double_free_error() {
    let mut pager = MemoryPager::new();
    let p = pager.allocate_page().unwrap();
    pager.free_page(p).unwrap();
    assert!(pager.free_page(p).is_err());
}

#[test]
fn freed_page_read_as_zero() {
    let mut pager = MemoryPager::new();
    let p = pager.allocate_page().unwrap();
    let mut page = Page::zeroed();
    page.as_slice_mut().fill(20);
    pager.write_page(p, &page).unwrap();
    pager.free_page(p).unwrap();
    let read = pager.read_page(p).unwrap();
    assert!(
        read.as_slice().iter().all(|&b| b == 0),
        "freed pages shouldnt leak"
    )
}

#[test]
pub fn stress_allocate_free() {
    let mut pager = MemoryPager::new();
    let mut allocated: Vec<u32> = Vec::new();
    //mass allocate
    for _ in 0..100 {
        allocated.push(pager.allocate_page().unwrap());
    }
    //mass free
    for &b in &allocated {
        pager.free_page(b).unwrap();
    }
    // check to see no new creation only reuse
    for _ in 0..100 {
        let p = pager.allocate_page().unwrap();
        assert!(allocated.contains(&p));
    }
    assert_eq!(pager.page_count(), 100, "Valid amount of pages");
}
