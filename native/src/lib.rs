use neon::prelude::*;
mod motor_drivers;

fn move_motor(mut cx: FunctionContext) {
    let motor = cx.argument::<JsNumber>(0)?;
    let off = cx.argument::<JsNumber>(1)?;
    motor_drivers::set_pwm(motor, &mut motor_drivers::get_controllers(), off)
}

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

register_module!(mut cx, {
    cx.export_function("hello", hello)
    cx.export_function("move_motor", move_motor)
});
