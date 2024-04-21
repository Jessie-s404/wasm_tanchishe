mod world;

// 通过 wasm-bindgen 导入必要的依赖项和类型
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};
use world::World; // 假定在 'world' 模块中定义了游戏逻辑

// 设置游戏逻辑更新的时间间隔为75毫秒
const TICK_MILLISECONDS: u32 = 75;

// 使用 wasm-bindgen 定义 Game 结构体，以便在 JS 中使用
#[wasm_bindgen]
pub struct Game {
    world: World, // 包含游戏世界状态的 World 结构体
    elapsed_milliseconds: u32, // 记录自上次更新以来经过的时间
}

#[wasm_bindgen]
impl Game {
    // 创建一个新的 Game 实例
    pub fn new() -> Game {
        Game {
            world: World::new(30, 30), // 初始化游戏世界，假设为30x30的网格
            elapsed_milliseconds: 0,
        }
    }

    // 游戏的“tick”方法，基于经过的时间来更新游戏状态
    pub fn tick(&mut self, elapsed_milliseconds: u32) {
        self.elapsed_milliseconds += elapsed_milliseconds;

        // 当累积的时间超过设定的间隔时，更新游戏世界状态并重置计时器
        if self.elapsed_milliseconds >= TICK_MILLISECONDS {
            self.elapsed_milliseconds = 0;
            self.world.tick(); // 调用 World 结构体的 tick 方法更新游戏状态
        }
    }

    // 渲染游戏画面到 Canvas
    pub fn render(&mut self, ctx: &CanvasRenderingContext2d) {
        // 创建 ImageData 对象，用于将 Rust 管理的像素数据传递到 JavaScript
        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&self.world.screen.pixel_buffer), // 使用 Clamped 确保数据在正确的范围内
            self.world.screen.width,
            self.world.screen.height,
        )
        .expect("应该从数组创建 ImageData");

        // 将 ImageData 对象绘制到 Canvas 上下文中
        ctx.put_image_data(&data, 0.0, 0.0)
            .expect("应该将数组写入上下文");
    }

    // 获取游戏画面的宽度
    pub fn width(&self) -> u32 {
        self.world.screen.width
    }

    // 获取游戏画面的高度
    pub fn height(&self) -> u32 {
        self.world.screen.height
    }

    // 处理鼠标点击事件，将点击坐标传递给 World 实例处理
    pub fn click(&mut self, x: i32, y: i32) {
        self.world.click(x, y); // 调用 World 结构体的 click 方法处理点击
    }
}
