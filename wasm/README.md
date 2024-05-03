# snake game
## build
```bash
wasm-pack build --target web
cd wwww
npm run dev # http://localhost:8080/ 
```


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
# add to package.json dependencies: "snake_game": "file:../pkg" 
npm install
# cargo add wee-alloc # - outdated
```
