# API 参考手册 (API Reference) - 最终版

## 核心概念：逻辑坐标系
MyGRAPHICS 自动处理 DPI 缩放。你传入的所有坐标（如 `[400.0, 300.0]`）均为**逻辑像素**。库内部会根据显示器的缩放因子（如 2.0x）自动转换为物理像素。

---

## 窗口与同步 (Lifecycle)

### `win.update(ms: u64)`
**同步绘图的核心：**
- **作用**：将当前帧在逻辑线程积累的所有 `draw_*` 指令打包发送给渲染线程，并阻塞逻辑线程指定的时间。
- **性能提示**：如果没有调用此函数，屏幕将不会有任何更新。

---

## 绘图指令 (Drawing Commands)

### `win.push_color_stack(color: [f32; 4])`
将颜色推入颜色栈，输入color为颜色，后续指令将自动应用此颜色（RGBA，范围 0.0-1.0）。

### `win.pull_color_stack()`
把颜色栈里的颜色推出。

### `win.draw_triangle(p1, p2, p3)`
绘制实心彩色三角形。

### `win.draw_line(p1, p2)`
绘制指定颜色和位置的线段。

### `win.draw_bezier(p1, p2, p3, p4)`
绘制基于四个控制点的三次贝塞尔曲线。

### `win.draw_text(text, pos)`
绘制一段文字。
- **优化**：库会自动缓存文字排版，内容相同时渲染速度极快。

---

## 输入处理 (Input Handling)

### `win.is_key_down(key: KeyCode) -> bool`
检查按键状态。

### `win.get_mouse_pos() -> [f32; 2]`
获取鼠标在窗口内的**逻辑坐标**。

---

## 获取窗口尺寸、以及设置窗口标题和尺寸

### `win.get_size() -> [f32; 2]`
获取窗口win的逻辑尺寸。

### `#[my_graphics::main(title = "my program", width = 800.0, height = 600.0)]`
设置窗口的标题和尺寸

## 示例程序

```rust
use my_graphics::KeyCode;

#[my_graphics::main]
fn main(mut win: my_graphics::window::Window) {
    loop {
        win.color_stack = [1.0, 0.0, 0.0, 1.0];
        win.draw_triangle([10.0, 10.0], [50.0, 10.0], [30.0, 50.0]);
        
        if win.is_key_down(KeyCode::Space) {
            println!("Space pressed!");
        }

        win.update(16); // 提交并同步
    }
}
```
