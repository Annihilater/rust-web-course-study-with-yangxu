mod utils;

// 把它下面内置的东西都引入进来
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// 在浏览器里面有个函数叫做 alert
// 如果想通过 WebAssembly 里面的 rust 代码调用这个 alert 函数，该怎么做呢？
// 应该先在 rust 文件里面像下面这样声明这个函数
// 这样 alert 这个函数就可以在 rust 代码里面调用了
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str); // 函数签名写一下，没有函数体
}

#[wasm_bindgen]
pub fn greet(s: &str) {
    // rust 代码里调用浏览器里面的 alert 函数
    // format 得到的类型是 string，需要调用 as_str() 转换成字符串切片
    alert(format!("Hello, {}!", s).as_str());
}
// 反过来，如果想在 js 里面调用 rust 里面的函数，就在函数上面加上 #[wasm_bindgen] 就可以了
