# Lab3实验报告

> 2017013640 张祎维 计71

#### 1. 回答问题：现有页面替换算法框架的实现存在问题，请解释为什么，并提供你的解决方案（自然语言表述即可，无需编程实现）

在`swap_out_one`函数中如下部分代码中：

```rust
if entry.accessed() {
    let swap_page: &mut [u8; PAGE_SIZE] =
    unsafe { frame.as_kernel_mut(PHYSICAL_MEMORY_OFFSET) };
    entry.set_target(disk_page_write(swap_page));
    entry.set_replaced(true);
}
```

当执行时钟页面替换算法等类似会将访问记录清零的算法时，此段代码将永久不会被执行到。故而不妨将`choose_victim`函数的返回值中添加一位参数表示`entry.accessed()`的值。

#### 2. 编程解决：实现时钟页面替换算法。

如题目要求，代码见`os/src/memory/page_replace/fifo.rs`

实验结果见`lab3.result`

关键部分代码如下：

```rust
fn choose_victim(&mut self) -> Option<(usize, Arc<Mutex<PageTableImpl>>)> {
    loop {
        let accessed = {
            let (vaddr, ref pt) = self.frames[0];
            let mut table = pt.lock();
            if let Some(entry) = table.get_entry(vaddr) {
                if entry.accessed() { entry.clear_accessed(); true } 
                else { false }
            } else { false }
        };
        if accessed {
            if let Some((vr, p)) = self.frames.pop_front() {
                self.frames.push_back((vr, p));
            }
        } else { return self.frames.pop_front(); }
    }
    None
}
```

即实现时钟页面替换算法：

首先对于每个页表项，确认其是否被访问过，判断`entry.accessed()`，若被访问过，则将此页表项访问记录清空执行`entry.clear_accessed()`，同时访问记录标记为`true`；否则标记为`false`。

之后判断访问记录，若为`true`，则按照时钟页面替换算法要求，将其推出`frames`后再加入`frames`，以此模仿指针转动；若为`false`，说明未曾被访问过，则可以直接将其返回，函数结束。

<!--注-->注：这种先得到访问记录标记之后再判断进行取舍的写法有得到计72程佳文指导；之前我的写法中将判断是否被访问过与是否返回结合在一起写了，这种写法无可避免会导致`dangling reference`问题，在与其交流后得到启发。

