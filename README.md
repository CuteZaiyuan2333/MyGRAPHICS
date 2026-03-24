# MyGRAPHICS!!!!! 🚀

**MyGRAPHICS!!!!!** 是一个基于 `wgpu` 构建的极简 Rust 图形库。它的核心目标是**极其简单**，让初学者或需要快速原型的开发者能够像写脚本一样编写图形程序，而不需要理解复杂的 GPU 管线或异步回调。注意：不要用于严肃项目，因为存在不可预测的稳定性问题。

## 🌟 核心特性

- **同步游戏循环**：摆脱复杂的闭包和回调，直接使用熟悉的 `loop { ... }` 结构。
- **状态驱动绘图**：类似经典 OpenGL 的状态机模式（`set_color` -> `draw`）。
- **极简 API**：只提供最基础、最有用的绘图和输入函数。
- **高清屏适配**：自动处理 HiDPI 缩放，逻辑坐标与物理像素自动映射。
- **高性能底层**：底层基于现代 `wgpu`，支持自动批处理（Batching）渲染。
- **全功能输入**：内置键盘按键、鼠标坐标、按键及滚轮的实时查询。
- **专业文字渲染**：集成 `glyphon`，支持系统字体和自定义字体文件。

## 🚀 快速开始

在你的 `Cargo.toml` 中添加依赖（假设已发布或本地引用）：

```toml
[dependencies]
my_graphics = { path = "path/to/my_graphics" }
```

编写你的第一个程序 `main.rs`：

```rust
use my_graphics::{self, KeyCode};

fn main() {
    let mut g = my_graphics::new("Hello MyGRAPHICS", 800, 600);

    loop {
        // 1. 逻辑与输入
        if g.is_key_down(KeyCode::Escape) { break; }
        let (mx, my) = g.get_mouse_pos();

        // 2. 绘图
        g.set_color([1.0, 0.0, 0.0, 1.0]); // 红色
        g.draw_triangle([mx, my], [mx + 50.0, my + 50.0], [mx - 50.0, my + 50.0]);

        // 3. 驱动渲染与事件
        g.update(16.0); // 约 60 FPS
    }
}
```

## 📂 运行示例

项目内置了多个演示程序：
- `cargo run --example demo`：基础动画与坐标演示。
- `cargo run --example font_test`：文字绘制与颜色演示。
- `cargo run --example input_test`：完整的输入控制演示。
