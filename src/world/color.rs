// 定义一个别名 `Rgb`，它是一个包含三个 u8（无符号8位整数）元素的数组，代表 RGB 颜色值
pub type Rgb = [u8; 3];

// 定义一个枚举 `Color`，它有四个变体，代表不同的颜色
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub(crate) enum Color {
    Background,
    Snake,
    Food,
    Fail,
}

// 实现从 `Color` 引用到 `Rgb` 类型的转换
impl From<&Color> for Rgb {
    fn from(color: &Color) -> Self {
        match color {
            Color::Background => [0; 3], // 背景色为黑色
            Color::Snake => [0, 255, 0], // 蛇的颜色为绿色
            Color::Food => [0, 0, 255],  // 食物的颜色为蓝色
            Color::Fail => [255, 0, 0],  // 失败的颜色为红色
        }
    }
}

// 实现从 `Rgb` 引用到 `Color` 类型的转换
impl From<&Rgb> for Color {
    fn from(rgb: &Rgb) -> Self {
        match rgb {
            [0, 0, 0] => Color::Background, // 黑色对应背景
            [0, 255, 0] => Color::Snake,     // 绿色对应蛇
            [0, 0, 255] => Color::Food,      // 蓝色对应食物
            [255, 0, 0] => Color::Fail,      // 红色对应失败
            _ => panic!("颜色不匹配"), // 如果颜色不匹配，则引发错误
        }
    }
}
