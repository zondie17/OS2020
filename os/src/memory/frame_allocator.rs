use crate::consts::MAX_PHYSICAL_PAGES;
use spin::Mutex;

/*pub struct SegmentTreeAllocator {
    a: [u8; MAX_PHYSICAL_PAGES << 1],
    m: usize,
    n: usize,
    offset: usize,
}

impl SegmentTreeAllocator {
    pub fn init(&mut self, l: usize, r: usize) {
        self.offset = l - 1;
        self.n = r - l;
        self.m = 1;
        while self.m < self.n + 2 {
            self.m = self.m << 1;
        }
        for i in (1..(self.m << 1)) {
            self.a[i] = 1;
        }
        for i in (1..self.n) {
            self.a[self.m + i] = 0;
        }
        for i in (1..self.m).rev() {
            self.a[i] = self.a[i << 1] & self.a[(i << 1) | 1];
        }
    }

    pub fn alloc(&mut self) -> usize {
        // assume that we never run out of physical memory
        if self.a[1] == 1 {
            panic!("physical memory depleted!");
        }
        let mut p = 1;
        while p < self.m {
            if self.a[p << 1] == 0 {
                p = p << 1;
            } else {
                p = (p << 1) | 1;
            }
        }
        let result = p + self.offset - self.m;
        self.a[p] = 1;
        p >>= 1;
        while p > 0 {
            self.a[p] = self.a[p << 1] & self.a[(p << 1) | 1];
            p >>= 1;
        }
        result
    }

    pub fn dealloc(&mut self, n: usize) {
        let mut p = n + self.m - self.offset;
        assert!(self.a[p] == 1);
        self.a[p] = 0;
        p >>= 1;
        while p > 0 {
            self.a[p] = self.a[p << 1] & self.a[(p << 1) | 1];
            p >>= 1;
        }
    }
}

pub static SEGMENT_TREE_ALLOCATOR: Mutex<SegmentTreeAllocator> = Mutex::new(SegmentTreeAllocator {
    a: [0; MAX_PHYSICAL_PAGES << 1],
    m: 0,
    n: 0,
    offset: 0,
});*/

pub struct FirstFitAllocator {
    a: [u8; MAX_PHYSICAL_PAGES],
    n: usize,
    offset: usize,
}

impl FirstFitAllocator {
    pub fn init(&mut self, l: usize, r: usize) {
        self.offset = l;
        self.n = r - l;
        for i in 0..self.n {
            self.a[i] = 0;
        }
    }

    pub fn alloc(&mut self, cnt: usize) -> Option<usize> {
        // assume that we never run out of physical memory
        // let mut pan = false;
        // for i in 0..self.n {
        //     if self.a[i] == 0 {
        //         pan = true;
        //     }
        // }
        // if pan == false {
        //     panic!("physical memory depleted!");
        // }
        let mut page = 0;
        let mut len = 0;
        let mut find = false;
        for i in 0..self.n {
            if self.a[i] == 1 {
                page = i+1;
                len = 0;
            } else {
                len += 1;
            }
            if len >= cnt && self.a[page] == 0 {
                find = true;
                break;
            }
        }

        return if find == true {
            for i in page..(page + cnt) {
                self.a[i] = 1;
            }
            Some(page + self.offset)
        } else {
            None
        }
    }

    pub fn dealloc(&mut self, f: usize, cnt: usize) {
        for i in f..(f+cnt) {
            // assert_eq!(self.a[i], 1);
            self.a[i - self.offset] = 0;
        }
    }

}

pub static FIRST_FIT_ALLOCATOR: Mutex<FirstFitAllocator> = Mutex::new(FirstFitAllocator {
    a: [0; MAX_PHYSICAL_PAGES],
    n: 0,
    offset: 0,
});