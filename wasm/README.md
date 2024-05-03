# snake game
## build
```bash
wasm-pack build --target web
cd wwww
npm run dev # http://localhost:8080/ 
```
## files
### [Cargo/toml](snake_game/Cargo.toml)
```toml
[dependencies]
wasm-bindgen = "0.2.92"

[lib]
crate-type = ["cdylib"]
```
### [lib.rs](snake_game/)
```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/www/utils/rnd.js")]
extern {
    fn rnd(max: usize) -> usize;
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum GameStatus { ... }
#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction { ... }

#[derive(Clone, Copy, PartialEq)]
pub struct SnakeCell(usize);
struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}
impl Snake {
    fn new(spawn_index: usize, size: usize) -> Snake { ... }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
    next_cell: Option<SnakeCell>,
    reward_cell: Option<usize>,
    status: Option<GameStatus>,
    points: usize,
}
#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_idx: usize) -> World { ... }
    pub fn points(&self) -> usize { ... }
    fn gen_reward_cell(max: usize, snake_body: &Vec<SnakeCell>) -> Option<usize> { ... }
    pub fn width(&self) -> usize { ... }
    pub fn reward_cell(&self) -> Option<usize> { ... }
    pub fn snake_head_idx(&self) -> usize { ... }
    pub fn game_status(&self) -> Option<GameStatus> { ... }
    pub fn game_status_text(&self) -> String { ... }
    pub fn change_snake_dir(&mut self, direction: Direction) { ... }
    pub fn snake_length(&self) -> usize { ... }
    pub fn snake_cells(&self) -> *const SnakeCell { ... }
    pub fn step(&mut self) { ... }
    pub fn start_game(&mut self) { ... }
    fn gen_next_snake_cell(&self, direction: &Direction) -> SnakeCell { ... }



```
### [index.ts](snake_game/www/index.ts)
```typescript
import init, { World, Direction, GameStatus } from "snake_game";
import { rnd } from "./utils/rnd";

init().then(wasm => {
  ...
  const canvas = <HTMLCanvasElement> document.getElementById("snake-canvas");
  const ctx = canvas.getContext("2d");
  ...
  gameControlBtn.addEventListener("click", _ => { ... });
  document.addEventListener("keydown", e => { ... });
  function drawWorld() { ... }
  function drawReward() { ... }
  function drawSnake() { ... }
  function drawGameStatus() { ... }
  function paint() { ... }
  function play() { ... }
}
```
### others
- [index.js.old](snake_game/www/index.js.old)
- [sum.wat](snake_game/www/sum.wat)
- [index.html](snake_game/www/index.html)
- [package.json](snake_game/www/package.json)
- [webpack.config.js](snake_game/www/webpack.config.js)
- [tsconfig.json](snake_game/www/tsconfig.json)
- [bootstrap.js](snake_game/www/bootstrap.js)


## wasm quickstart
- start from "web assembly start" commit https://github.com/Jerga99/snake-rust-game/commits/master/?before=5ceb15dee6f27f6880ae231df2b710b79c2b3dca+70
- wat2wasm https://webassembly.github.io/wabt/demo/wat2wasm/
- [sum.wat](snake_game/www/sum.wat)
```javascript
const wasmInstance = new WebAssembly.Instance(wasmModule, {});
const { sum } = wasmInstance.exports;
for (let i = 0; i < 10; i++) {
  console.log(sum(i, i));
}
```
```bash
mv ~/Downloads/test.wasm public/sum.wasm   
xxd -g1 sum.wasm
```
- https://www.rapidtables.com/convert/number/hex-to-decimal.html
- js: `debugger` 
  
## webpack dev server
```bash
cd wwww
npm init -y
npm install --save webpack webpack-cli copy-webpack-plugin
npm install --save-dev webpack-dev-server
npm install --save typescript ts-loader
npm run dev # http://localhost:8080/  , http://localhost:8080/webpack-dev-server
nmp run build
```
## cargo
```bash
cargo add wasm-bindgen
cargo install wasm-pack
wasm-pack build --target web # creates pkg folder
npm install  # one time - after add to package.json dependencies: "snake_game": "file:../pkg" 
# cargo add wee-alloc # - outdated
```
