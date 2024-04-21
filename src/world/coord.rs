// 引入 Add trait，用于重载 '+' 运算符
use std::ops::Add;

// 为 Coord 结构体派生 Debug（用于格式化输出），PartialEq（用于比较），Clone 和 Copy 特性
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Coord {
    pub x: i32, // x 坐标
    pub y: i32, // y 坐标
}

// 实现 Coord 结构体
impl Coord {
    // 构造函数，创建一个新的 Coord 实例
    pub fn new(x: i32, y: i32) -> Coord {
        Coord { x, y }
    }
}

// 为 Coord 实现 Add trait，使两个 Coord 实例可以相加
impl Add<Coord> for Coord {
    type Output = Coord; // 定义加法操作的返回类型为 Coord

    // 实现加法操作，将两个坐标点相加得到一个新的 Coord 实例
    fn add(self, rhs: Coord) -> Self::Output {
        Coord {
            x: self.x + rhs.x, // 新的 x 坐标是两个 x 坐标之和
            y: self.y + rhs.y, // 新的 y 坐标是两个 y 坐标之和
        }
    }
}

// 实现从元组 (i32, i32) 到 Coord 的转换
impl From<(i32, i32)> for Coord {
    // 实现转换方法
    fn from((x, y): (i32, i32)) -> Self {
        Coord::new(x, y) // 使用 new 方法创建 Coord 实例
    }
}
