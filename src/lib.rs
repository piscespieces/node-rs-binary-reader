use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    let s: Handle<JsString> = cx.argument(0)?;

    let contents = s.value(&mut cx);

    Ok(cx.string(contents))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    Ok(())
}

// fn get_num_cpus(mut cx: FunctionContext, text: String) -> JsResult<JsNumber> {
//     println!("{}", text);
//     Ok(cx.number(num_cpus::get() as f64))
// }