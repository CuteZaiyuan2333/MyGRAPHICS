# 实现计划 (Implementation Plan)

## 阶段一：原型验证 (Prototype)
- [ ] **项目搭建**：初始化 `Cargo.toml`。
- [ ] **跨线程骨架**：
    - 实现 `#[my_graphics::main]` 宏（或使用函数式封装）。
    - 验证 `winit` 在主线程运行，用户 `main` 在子线程运行。
    - 建立基于 `crossbeam-channel` 的简单指令传递系统。

## 阶段二：渲染核心 (WGPU Backend)
- [ ] **初始化 WGPU**：创建渲染实例、适配器和窗口表面。
- [ ] **三角形管线**：实现能够接受位置和颜色输入的顶点着色器和渲染管线。
- [ ] **指令分发**：实现 `DrawCmd` 的处理逻辑，将其转化为 WGPU 的 `Queue::write_buffer` 和 `CommandEncoder` 操作。

## 阶段三：功能增强 (Features)
- [ ] **输入同步**：
    - 在渲染线程捕获 `WindowEvent::KeyboardInput` 和 `WindowEvent::CursorMoved`。
    - 将数据写入 `Arc<RwLock<InputState>>`，并在用户侧提供便捷方法。
- [ ] **颜色栈**：在 `WindowProxy` 中实现颜色跟踪。
- [ ] **字符渲染集成**：
    - 集成 `glyphon`。
    - 创建专用的文本渲染 pass。

## 阶段四：优化与完善 (Refinement)
- [ ] **资源释放**：处理窗口关闭时的线程安全退出。
- [ ] **性能调优**：实现简单的顶点批处理，减少 Draw Call。
- [ ] **示例应用**：编写 1-2 个简单的演示程序（如简单的贪吃蛇或雷达图绘制）。

## 依赖库 (Dependencies)
```toml
[dependencies]
winit = "0.28"
wgpu = "0.17"
crossbeam-channel = "0.5"
bytemuck = { version = "1.13", features = ["derive"] }
glyphon = "0.3"
parking_lot = "0.12" # 更高性能的锁
```
