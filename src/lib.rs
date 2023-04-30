use neon::prelude::*;

fn pid_path(mut cx: FunctionContext) -> JsResult<JsString> {
    let pid = cx.argument::<JsNumber>(0)?.value(&mut cx);

    return match darwin_libproc::pid_path(pid as i32) {
        Ok(pid_path) => match pid_path.into_os_string().into_string() {
            Ok(pid_path) => Ok(cx.string(pid_path)),
            Err(_) => {
                // TODO Would be nice if a loosy copy was provided on the error
                cx.throw_error("Cannot represent path in Unicode without data loss.")
            },
        },
        Err(err) => cx.throw_error(err.to_string()),
    };
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("pidPath", pid_path)?;
    Ok(())
}
