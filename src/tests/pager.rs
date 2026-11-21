/*                                     INTEGRATION TESTING FOR PAGER              */
use crate::pager::{Page, Pager, memory::MemoryPager};
#[test]
fn allocate_and_read_page() {
    let mut pager = MemoryPager::new();

    // Verify that consecutive allocations return distinct page IDs
    let p1 = pager.allocate_page().unwrap();
    let p2 = pager.allocate_page().unwrap();
    assert_ne!(p1, p2);

    // Prepare a page with test data
    let mut page = Page::zeroed();
    page.as_slice_mut()[0] = 56;
    page.write_u32(100, 0xDEADBEEF);

    // Write to p1 and read it back
    pager.write_page(p1, &page).unwrap();
    let read_p1 = pager.read_page(p1).unwrap();

    assert_eq!(read_p1.as_slice()[0], 56);
    assert_eq!(read_p1.read_u32(100), 0xDEADBEEF);

    // Ensure p2 remains isolated and untouched (zeroed)
    let read_p2 = pager.read_page(p2).unwrap();
    assert_eq!(read_p2.as_slice()[0], 0);
    assert_eq!(read_p2.read_u32(100), 0);
}
