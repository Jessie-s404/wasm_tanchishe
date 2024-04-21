// 导入相关模块
mod coord;
mod color;
mod screen;

// 使用语句，引入模块中的类型
use self::coord::Coord;
use self::color::Color;
use self::screen::Screen;
use rand::Rng; // 使用 rand crate 中的 Rng trait
use std::collections::VecDeque; // 使用标准库中的 VecDeque 类型

// 定义起始蛇的长度
const START_LEN: i32 = 7;
// 定义不变量，表示蛇的长度始终大于0
const INVARIANT: &str = "蛇的长度始终大于0";

// 定义游戏世界的结构体
pub struct World {
    pub screen: Screen, // 游戏屏幕，负责绘图
    direction: Coord, // 当前蛇的前进方向
    snake: VecDeque<Coord>, // 代表蛇身体的一系列坐标
    alive: bool, // 游戏是否处于活动状态
}

// World 结构体的实现
impl World {
    // 构造函数，初始化游戏世界
    pub fn new(width: u32, height: u32) -> World {
        let mut world = World {
            screen: Screen::new(width, height), // 初始化屏幕
            direction: (1, 0).into(), // 初始方向向右
            snake: VecDeque::new(), // 初始化蛇的身体
            alive: true, // 开始时蛇是活的
        };

        // 清屏，创建初始的蛇和食物
        world.screen.clear();
        world.create_initial_snake();
        world.create_initial_food();
        world
    }

    // 游戏逻辑的“tick”方法，每次调用都更新游戏状态
    pub fn tick(&mut self) {
        if self.alive {
            // 如果蛇活着，计算新的头部位置
            let new_head = self.get_new_head();
            // 获取新头部位置的颜色，以确定蛇下一步的动作
            let new_head_pixel = self.screen.get_color_at(&new_head);

            // 把蛇的头部移动到新位置
            self.extend_head_to(&new_head);

            // 根据新头部位置的像素颜色决定蛇的动作
            match new_head_pixel {
                Color::Food => self.create_food(), // 吃到食物，创建新的食物
                Color::Snake => self.die(), // 吃到自己，游戏结束
                _ => self.shorten_tail(), // 没有吃到食物，移动蛇身
            }
        }
    }

    // 处理点击事件，改变蛇的移动方向
    pub fn click(&mut self, x: i32, y: i32) {
        if self.alive {
            let head = self.snake.back().expect(INVARIANT);

            // 根据点击位置和蛇头的位置确定新的移动方向
            self.direction = match self.direction.x {
                // 如果当前水平方向没有移动，则改变水平方向
                0 => (if x < head.x { -1 } else { 1 }, 0),
                // 如果当前垂直方向没有移动，则改变垂直方向
                _ => (0, if y < head.y { -1 } else { 1 }),
            }
            .into();
        } else {
            // 如果蛇死了，点击屏幕任何位置都会重置游戏
            self.reset_game()
        }
    }

    // 清空屏幕并重新开始游戏
    fn reset_game(&mut self) {
        self.direction = (1, 0).into();
        self.snake = VecDeque::new();
        self.alive = true;
        self.screen.clear();
        self.create_initial_snake();
        self.create_initial_food();
    }

    // 创建初始蛇
    fn create_initial_snake(&mut self) {
        let start_y = self.screen.height as i32 / 2;
        for x in 0..START_LEN {
            self.screen.set_color_at(&(x, start_y).into(), Color::Snake);
            self.snake.push_back((x, start_y).into());
        }
    }

    // 创建初始食物
    fn create_initial_food(&mut self) {
        let initial_food_y = self.screen.height as i32 / 2 - 2;
        self.screen
            .set_color_at(&(START_LEN, initial_food_y).into(), Color::Food);
    }

    // 计算新的蛇头位置
    fn get_new_head(&self) -> Coord {
        let screen_width = self.screen.width;
        let screen_height = self.screen.height;
        let moved_head = *self.snake.back().expect(INVARIANT) + self.direction;
        let x = (moved_head.x + screen_width as i32) % screen_width as i32;
        let y = (moved_head.y + screen_height as i32) % screen_height as i32;
        (x, y).into()
    }

    // 将蛇头移动到新位置
    fn extend_head_to(&mut self, new_head: &Coord) {
        self.screen.set_color_at(new_head, Color::Snake);
        self.snake.push_back(*new_head);
    }

    // 缩短蛇尾
    fn shorten_tail(&mut self) {
        let tail = self.snake.pop_front().expect(INVARIANT);
        self.screen.set_color_at(&tail, Color::Background);
    }

    // 创建新的食物
    fn create_food(&mut self) {
        let pixel_count = self.screen.pixel_count as usize;
        let random_skip = rand::thread_rng().gen_range(0..pixel_count) as usize;

        let coord = self
            .screen
            .iter_pixels()
            .filter(|(color, _)| *color == Color::Background)
            .map(|(_, coord)| coord)
            .collect::<Vec<_>>()
            .into_iter()
            .cycle()
            .skip(random_skip)
            .next()
            .expect("至少有一个像素应该是空闲的");

        self.screen.set_color_at(&coord, Color::Food);
    }

    // 游戏结束处理
    fn die(&mut self) {
        self.alive = false;
        self.screen.set_color_at_edges(Color::Fail);
    }
}
