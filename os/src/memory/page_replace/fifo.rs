use {
    super::*,
    alloc::{collections::VecDeque, sync::Arc},
    spin::Mutex,
};

#[derive(Default)]
pub struct FifoPageReplace {
    frames: VecDeque<(usize, Arc<Mutex<PageTableImpl>>)>,
}

impl PageReplace for FifoPageReplace {
    fn push_frame(&mut self, vaddr: usize, pt: Arc<Mutex<PageTableImpl>>) {
        println!("push vaddr: {:#x?}", vaddr);
        self.frames.push_back((vaddr, pt));
    }

    fn choose_victim(&mut self) -> Option<(usize, Arc<Mutex<PageTableImpl>>)> {
        // 选择一个已经分配的物理页帧
        // self.frames.pop_front()
        loop {
/*            let (vaddr, ref pt) = self.frames[0];
            let mut table = pt.lock();
            if let Some(entry) = table.get_entry(vaddr) {
                if entry.accessed() {
                    entry.clear_accessed();
                    let Some((vr, p)) = self.frames.pop_front();
                    self.frames.push_back((vr, p));
                } else {
                    return self.frames.pop_front();
                }
            }*/
            let accessed = {
                let (vaddr, ref pt) = self.frames[0];
                let mut table = pt.lock();
                if let Some(entry) = table.get_entry(vaddr) {
                    if entry.accessed() {
                        entry.clear_accessed();
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            };
            if accessed {
                if let Some((vr, p)) = self.frames.pop_front() {
                    self.frames.push_back((vr, p));
                }
            } else {
                return self.frames.pop_front();
            }
        }
        None
    }

    fn tick(&self) {}
}
