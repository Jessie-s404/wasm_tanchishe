// 引入 color 模块中的 Color 和 Rgb 结构
use super::color::{Color, Rgb};
// 引入 coord 模块中的 Coord 结构
use super::coord::Coord;

// 定义每个像素占用的字节数
const BYTES_PER_PIXEL: u32 = 4;

// Screen 结构体表示画布，包含像素数据、像素总数以及屏幕的宽度和高度
pub struct Screen {
    pub pixel_buffer: Vec<u8>, // 存储屏幕上所有像素颜色值的缓冲区
    pub pixel_count: u32,      // 屏幕上的像素总数
    pub width: u32,            // 屏幕宽度
    pub height: u32,           // 屏幕高度
}

impl Screen {
     // 创建一个新的 Screen 实例
   pub fn new(width: u32, height: u32) -> Self {
        let pixel_count = width * height; // 计算像素总数
        let screen_size_in_bytes = pixel_count * BYTES_PER_PIXEL; // 计算缓冲区的大小
        Self {
            pixel_count,
            width,
            height,
            pixel_buffer: vec![255u8; screen_size_in_bytes as usize], // 初始化所有像素为白色
        }
    }
    // 清除屏幕，将所有像素设置为背景颜色
   pub fn clear(&mut self) {
        self.iter_coords().for_each(|coord| {
            self.set_color_at(&coord, Color::Background);
        });
    }
    // 在指定坐标上设置颜色
   pub fn set_color_at(&mut self, coord: &Coord, color: Color) {
        // 计算坐标对应的缓冲区索引
        let i = self.get_buffer_index_for(coord);
        // 设置颜色
        self.pixel_buffer[i..i + 3].copy_from_slice(Rgb::from(&color).as_slice());
    }
    // 在屏幕的边缘设置颜色
   pub fn set_color_at_edges(&mut self, color: Color) {
        let screen_width = self.width as i32;
        let screen_height = self.height as i32;
        // 过滤出边缘的坐标并设置颜色
        self.iter_coords()
            .filter(|Coord { x, y }| {
                *x == 0 || *y == 0 || *x == screen_width - 1 || *y == screen_height - 1
            })
            .for_each(move |coord| self.set_color_at(&coord, color));
    }
    // 获取指定坐标上的颜色
   pub fn get_color_at(&self, coord: &Coord) -> Color {
        // 计算坐标对应的缓冲区索引
        let i = self.get_buffer_index_for(coord);
        (&[
            self.pixel_buffer[i],
            self.pixel_buffer[i + 1],
            self.pixel_buffer[i + 2],
        ])
            .into()
    }
    // 根据坐标计算缓冲区的索引
   pub fn get_buffer_index_for(&self, Coord { x, y }: &Coord) -> usize {
        (*y as usize * self.width as usize + *x as usize) * BYTES_PER_PIXEL as usize
    }
    // 生成一个迭代器，用于迭代屏幕上的所有坐标
   pub fn iter_coords(&self) -> impl Iterator<Item = Coord> {
        let width = self.width;
        let height = self.height;
        // 创建一个迭代器，生成屏幕上所有点的坐标
        (0..height as i32).flat_map(move |y| (0..width as i32).map(move |x| (x, y).into()))
    }
    // 创建一个迭代器，生成屏幕上所有点的坐标
   pub fn iter_pixels(&self) -> impl Iterator<Item = (Color, Coord)> + '_ {
        self.iter_coords()
            .map(|coord: Coord| (self.get_color_at(&coord), coord))
    }
}

