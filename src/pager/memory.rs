/*                                           MEMORY PAGER                                      */
use super::*;
use crate::error::*;

pub struct MemoryPager {
    pages: Vec<Page>,
    freelist: Vec<PageNo>,
}

impl MemoryPager {
    pub fn new() -> Self {
        Self {
            pages: Vec::new(),
            freelist: Vec::new(),
        }
    }
}

impl Pager for MemoryPager {
    ///Read a page and return a Result<T,Err> where T is a reference to page and Err is catched errors.
    /// Input:PageNo
    fn read_page(&self, page_no: PageNo) -> Result<&Page> {
        //check if out of bound or header
        if page_no == 0 || page_no as usize > self.pages.len() {
            return Err(Error::PageOutOfRange(page_no));
        }
        //return the page
        Ok(&self.pages[page_no as usize - 1])
    }

    ///Write a page and return a Result<T,Err> where T is a reference to page and Err is catched errors
    /// Input: PageNo,an updated page reference
    fn write_page(&mut self, page_no: PageNo, page: &Page) -> Result<()> {
        //check if out of bound or header
        if page_no == 0 || page_no as usize > self.pages.len() {
            return Err(Error::PageOutOfRange(page_no));
        }

        //update the page with new data
        self.pages[page_no as usize - 1] = page.clone();
        Ok(())
    }

    ///Allocates a page by reusing a freed page or creating a new page returning a Result<T,Err> where T is PageNo and Err is captured errors
    fn allocate_page(&mut self) -> Result<PageNo> {
        // check if their is an available page
        if let Some(page_no) = self.freelist.pop() {
            self.pages[page_no as usize - 1] = Page::zeroed();
            return Ok(page_no);
        }

        //if not create a new page
        self.pages.push(Page::zeroed());
        Ok(self.pages.len() as PageNo)
    }

    ///Frees a page by removing data from that page return Result<(),Err> where Err is captured errors
    fn free_page(&mut self, page_no: PageNo) -> Result<()> {
        //check if out of bound or header
        if page_no == 0 || page_no as usize > self.pages.len() {
            return Err(Error::PageOutOfRange(page_no));
        }

        //check if page is in freelist already
        if self.freelist.contains(&page_no) {
            return Err(Error::DoubleFree(page_no));
        }

        //free up the page
        self.pages[page_no as usize - 1] = Page::zeroed();
        self.freelist.push(page_no);
        Ok(())
    }

    ///returns total amount of pages
    fn page_count(&self) -> u32 {
        self.pages.len() as u32
    }
}
