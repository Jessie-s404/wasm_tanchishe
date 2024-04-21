// 引入 wasm 模块，并导出 Game 类
import init, { Game } from "./pkg/game.js";

// 获取 canvas 元素
const canvas = document.getElementById("canvas");

// 记录上一帧的时间戳
let lastFrame = Date.now();

// 初始化 wasm 模块，并在初始化完成后执行回调函数
init().then(() => {
  // 创建 Game 实例
  const game = Game.new();

  // 设置 canvas 尺寸为游戏尺寸
  canvas.width = game.width();
  canvas.height = game.height();

  // 为 canvas 添加点击事件监听器
  canvas.addEventListener("click", (event) => onClick(game, event));

  // 使用 requestAnimationFrame 启动游戏循环
  requestAnimationFrame(() => onFrame(game));
});

// 游戏循环的回调函数
function onFrame(game) {
  // 计算帧间隔时间
  const delta = Date.now() - lastFrame;
  lastFrame = Date.now();

  // 更新游戏状态
  game.tick(delta);

  // 渲染游戏画面
  game.render(canvas.getContext("2d"));

  // 使用递归调用 requestAnimationFrame，实现持续的游戏循环
  requestAnimationFrame(() => onFrame(game));
}

// 处理点击事件的回调函数
function onClick(game, event) {
  // 获取点击位置相对于 canvas 的坐标
  const rect = event.target.getBoundingClientRect();
  const x = ((event.clientX - rect.left) / rect.width) * game.width();
  const y = ((event.clientY - rect.top) / rect.height) * game.height();

  // 将点击事件传递给游戏对象处理
  game.click(x, y);
}
