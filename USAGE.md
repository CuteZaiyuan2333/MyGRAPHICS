# MyGRAPHICS!!!!! 语法与 API 手册

本文档详细介绍了 `MyGRAPHICS!!!!!` 的所有公开 API 及其设计逻辑。

## 1. 核心概念

### 同步循环 (The Loop)
不同于大多数 Rust 图形库强制接管 `main` 函数，本库允许你保留 `main` 的控制权。
**必须注意：** 每一帧循环必须调用一次 `graphics.update(ms)`。该函数承担了三个任务：
1. **渲染**：将这一帧积压的所有绘图指令发给 GPU。
2. **事件轮询**：获取最新的鼠标、键盘、窗口缩放事件。
3. **帧率控制**：根据传入的时间参数自动休眠，防止 CPU 占用过高。

### 坐标系 (Coordinate System)
本库使用**逻辑像素坐标系**：
- **(0, 0)**：窗口左上角。
- **(Width, Height)**：窗口右下角。
- **HiDPI**：在高分屏下，坐标会自动缩放。如果你设定窗口为 400x300，那么坐标 400 永远代表窗口边缘（假设你没有手动拉动窗口），无论实际物理像素是多少。

---

## 2. API 详解

### 窗口管理
- `my_graphics::new(title: &str, width: u32, height: u32) -> Graphics`
  创建一个新窗口并初始化绘图上下文。
- `graphics.get_size() -> (u32, u32)`
  获取当前窗口的逻辑尺寸。

### 绘图状态设置 (State)
本库是状态驱动的。设置后的状态会持续有效，直到被修改。
- `graphics.set_color([r, g, b, a]: [f32; 4])`
  设置后续绘图的颜色。
- `graphics.set_picture(path: &str)`
  加载并设置一张贴图。设置贴图后，后续的三角形将显示该图片的内容。

### 几何绘制
- `graphics.draw_triangle(p1: [f32; 2], p2: [f32; 2], p3: [f32; 2])`
  绘制一个三角形。这是最基础的绘图单元，矩形、多边形均可用其拼凑。

### 文字绘制
- `graphics.set_font(family: &str)`
  设置系统字体家族（如 `"serif"`, `"sans-serif"`, `"Arial"`）。
- `graphics.set_font_path(path: &str)`
  从文件加载 `.ttf` 或 `.otf` 字体。
- `graphics.draw_char(character: char, pos: [f32; 2], size: f32)`
  在指定位置绘制单个字符。

### 输入查询 (Input)
- **键盘**：
  - `graphics.is_key_down(key: KeyCode) -> bool`：按键是否正被按住。
  - `graphics.get_last_key() -> Option<KeyCode>`：获取最近一次按下的键。
  - `graphics.get_pressed_keys() -> Vec<KeyCode>`：获取当前所有按下的键。
- **鼠标**：
  - `graphics.get_mouse_pos() -> (f32, f32)`：获取鼠标逻辑坐标。
  - `graphics.is_mouse_down(button: MouseButton) -> bool`：鼠标键是否按住。
  - `graphics.get_mouse_wheel() -> f32`：获取自上一帧以来的滚轮滚动量。
- **注意**：
  - 当前存在极其严重的稳定性问题，已知按下caps lock时按下按键会闪退，或按下fn按键时按下其他按键会闪退。千万不要用于严肃项目。
---

## 3. 进阶技巧

### 实现清屏 (Clear Screen)
由于库每帧渲染前会自动刷新缓冲区，你只需要在循环开始处画一个覆盖全屏的矩形即可实现自定义背景色：
```rust
let (w, h) = g.get_size();
g.set_color([0.2, 0.2, 0.2, 1.0]); // 深灰色背景
g.draw_triangle([0.0, 0.0], [w as f32, 0.0], [0.0, h as f32]);
g.draw_triangle([w as f32, 0.0], [w as f32, h as f32], [0.0, h as f32]);
```

### 性能优化
库内部实现了自动批处理。**连续调用相同颜色或相同贴图的绘制函数**会被合并为一次 GPU 提交。建议尽量减少在循环中频繁交替切换贴图的操作。
