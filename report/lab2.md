# Lab2实验报告

> 2017013640 张祎维 计71

#### 1. 回答：如果 OS 无法提前知道当前硬件的可用物理内存范围，请问你有何办法让 OS 获取可用物理内存范围？

利用`linux`设备树文件`DTB`文件。

`DTB`文件中的`off_meme_rsvmap`中存储着`memory reservation block`的偏移地址，即可据此获取可用物理内存范围。

#### 2. 编程：实现 `FirstFitAllocator` ，接口参考 `SegmentTreeAllocator` ，并完成内部实现（可参考 [ucore](https://github.com/LearningOS/ucore_os_lab/blob/master/labcodes_answer/lab2_result/kern/mm/default_pmm.c#L122) 中的算法）。

代码见`os/src/memory/frame_allocator.rs`及`os/src/memory/mod.rs`

试验结果见`lab2.result`

`frame_allocator.rs`关键部分代码：

```rust
pub fn alloc(&mut self, cnt: usize) -> Option<usize> {
    let mut page = 0;
    let mut len = 0;
    let mut find = false;
    for i in 0..self.n {
        if self.a[i] == 1 { page = i+1; len = 0; } 
        else { len += 1; }
        if len >= cnt && self.a[page] == 0 { find = true; break; }
    }
    return if find == true {
        for i in page..(page + cnt) { self.a[i] = 1; }
        Some(page + self.offset)
    } else { None }
}
```

即`page`为起始页面号，当`page`之后连续的可用页面数量`len`多于`cnt`时，即可中止搜索并返回。

计算`page`和`len`策略如下：遇到`1`(已被占用的页面)时，将`page`更新为当前页面号+1，且`len`清零；否则则将`len`+1，即连续可用页面数量+1。之后当`len`>=​`cnt`且当前页面为可用页面时，即可退出搜索，并将结果返回。若到最后都没有找到足够的空间，则返回`None`。

