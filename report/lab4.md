# Lab4实验报告

> 2017013640 张祎维 计71

#### 1. 回答：详细描述第六章文档中 `process::init` 的执行过程。（4 分）

首先获取当前自身的内核线程`boot_thread`，之后创建一个新的临时线程`temp_thread`，再利用`boot_thread`作为参数初始化`temp_thread`线程；之后调用`switch_to`函数，从`boot_thread`线程切换到`temp_thread`线程，在这个临时线程中打印

```rust
"I'm leaving soon, but I still want to say: Hello world!"
```

之后再切换回其启动线程`boot_thread`，执行`init`函数中的后续打印

```rust
"switched back from temp_thread!"
```

之后进入空循环。

#### 2. 回答：给出 `switch` 时，重要寄存器的使用情况，画出栈的使用情况。（6 分）

进入`switch`之后，开始时sp指向一个`trapframe`，即初始内核栈：

| ...           |
| ------------- |
| trapframe(sp) |

之后在当前栈上分配内存空间以保存CPU状态，要保存的为返回地址ra，页表寄存器satp，和被调用者保存寄存器 $s_0$~$s_{11}$ 共14个寄存器，即开辟$14\times8$的空间：

| ...  |
| ---- |
| 14*8 |
| sp   |

之后，更新当前线程栈顶地址，同时依次保存各寄存器的值

| ...    |
| ------ |
| ra     |
| satp   |
| s0     |
| s1     |
| s2     |
| s3     |
| s4     |
| s5     |
| s6     |
| s7     |
| s8     |
| s9     |
| s10    |
| s11    |
| sp(a0) |

而在同一进程内的时候，satp寄存器中的值不会改变，因为共享一个页表。

在保存时，首先保存返回地址ra，之后依序保存 $s_0$~$s_{11}$ 寄存器，之后将satp寄存器中的内容保存到$s_{11}$寄存器中，在将其保存到栈中。

之后读取要切换到的线程栈顶地址，并直接换栈，即读取要切换到的线程栈顶地址，其位于$a_1$寄存器中，将其保存到sp寄存器中。

| ...    |
| ------ |
| ra     |
| satp   |
| s0     |
| s1     |
| s2     |
| s3     |
| s4     |
| s5     |
| s6     |
| s7     |
| s8     |
| s9     |
| s10    |
| s11    |
| sp(a1) |

之后依序读取恢复各寄存器。

如前保存过程般，首先将$satp$寄存器中的内容保存到$s_{11}$寄存器中，之后读取到satp寄存器中，之后刷新TLB。接着依次读取ra和 $s_0$~$s_{11}$ 寄存器中的内容。

之后出栈，在当前栈上回收用来保存线程状态的内存，$sp = sp + 14\times8$。

| ...  |
| ---- |
| sp   |

