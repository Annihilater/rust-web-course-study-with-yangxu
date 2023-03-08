// mod utils;
pub mod errors;
pub mod models;

use models::course::{delete_course, get_courses_by_teacher, Course};
use serde_json::to_string;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::*;
use web_sys::HtmlButtonElement;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen]
// extern {
//     fn alert(s: &str);
// }

// #[wasm_bindgen]
// pub fn greet() {
//     alert("Hello, wasm-client!");
// }

// extern "C" 表示使用 ApplicationBinaryInterface 调用程序二进制接口
// 然后申明了两个函数，这两个函数都是浏览器里面的函数
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    // 声明 confirm 函数，就是浏览器弹出的确认弹窗
    fn confirm(s: &str) -> bool;

    // 这个 log 就是浏览器里面可以写日志的 console.log()
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-client!")
}

// 主函数，项目已启动就会执行里面的代码，可以是异步的也可以不是异步的
// 这里的作用是把从 webservice 获取到的课程，生成相应的页面元素
#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global window exists");
    let document = window.document().expect("no global document exists");

    let left_tbody = document
        .get_element_by_id("left-tbody")
        .expect("left div not exists");

    let courses: Vec<Course> = models::course::get_courses_by_teacher(1).await.unwrap();
    for c in courses.iter() {
        let tr = document.create_element("tr")?;
        tr.set_attribute("id", format!("tr-{}", c.id).as_str())?;
        let td = document.create_element("td")?;
        td.set_text_content(Some(format!("{}", c.id).as_str()));
        tr.append_child(&td)?;

        let td = document.create_element("td")?;
        td.set_text_content(Some(c.name.as_str()));
        tr.append_child(&td)?;

        let td = document.create_element("td")?;
        td.set_text_content(Some(c.time.format("%Y-%m-%d").to_string().as_str()));
        tr.append_child(&td)?;

        let td = document.create_element("td")?;
        if let Some(desc) = c.description.clone() {
            td.set_text_content(Some(desc.as_str()));
        }
        tr.append_child(&td)?;

        let td = document.create_element("td")?;
        // WebAssembly 第一节结束的时候写的代码，第二节的时候有变动，把这部分注释掉了重写
        // let btn = document.create_element("button")?;
        // btn.set_attribute("class", "btn btn-danger btn-sm")?;
        // btn.set_text_content(Some("Delete"));
        // td.append_child(&btn)?;
        // tr.append_child(&td)?;

        let btn: HtmlButtonElement = document
            .create_element("button")
            .unwrap()
            .dyn_into::<HtmlButtonElement>()
            .unwrap();

        let cid = c.id.clone();
        let click_closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
            let r = confirm(format!("确认删除 ID 为 {} 的课程?", cid).as_str());
            match r {
                // 弹窗用户确认，如果返回的是 true 则执行删除操作，并弹窗提示用户删除成功
                // 删除操作调用的是 delete_course 函数，这个函数是异步的，而目前一般情况下用闭包调用异步的函数，那么闭包也必须是异步的
                // 但是目前 rust 里面异步的闭包是不稳定的，写上 async 关键字会报一些错误
                // 所以这里使用 spawn_local() 函数，它来自于 wasm_bindgen_futures::*
                true => {
                    // delete_course(1, cid) 是一个异步函数，没有写 await，所以说返回的是一个 future，一个未来
                    // 而调用 spawn_local(未来) 就会把 future 的执行放在当前线程，使用这种方式就可以在普通的闭包函数里面执行异步的函数了
                    spawn_local(delete_course(1, cid));
                    alert("删除成功！");
                    // 最后让浏览器刷新一下
                    web_sys::window().unwrap().location().reload().unwrap();
                }
                // 如果返回的不是 true，则什么都不做
                _ => {}
            }
        }) as Box<dyn Fn(_)>);

        // add_event_listener_with_callback() 的第二个参数要求的是一个 function 类型的引用
        // 怎么上面的闭包转为 function 类型的引用呢？
        // 首先把闭包放在 Box::new() 到智能指针里面，然后外面使用 Closure::wrap() 包一下，转成 Box，实现了 Fn trait (就是 Box<dyn Fn(_)>)
        // 然后再调用 as_ref().unchecked_ref()，才能把它这个闭包转化成想要的 function 类型的引用，最后还需要调用一下 forget() 方法。
        // 因为我们创建的闭包在走出作用域的时候，会被 drop 掉（会被丢弃），正常情况下，闭包对应的回调函数也会失效。
        // 而调用 forget() 方法之后，当闭包走出作用域的时候，闭包的回调函数依然有效，当然这也会造成内存泄露，所以这一块需要仔细想一下
        btn.add_event_listener_with_callback("click", click_closure.as_ref().unchecked_ref())?;
        click_closure.forget();

        btn.set_attribute("class", "btn btn-danger btn-sm")?;
        btn.set_text_content(Some("Delete"));
        td.append_child(&btn)?;
        tr.append_child(&td)?;

        left_tbody.append_child(&tr)?;
    }

    Ok(())
}
