use druid::{AppLauncher,WindowDesc,Widget, PlatformError};
use druid::widget::Label;


fn main() -> Result<(), PlatformError> {
let main_windows = WindowDesc::new(build_ui);
AppLauncher::with_window(main_windows).launch(())?;
Ok(())
}

fn build_ui() -> impl Widget<()> {
    Label::new("hello druid!")
}
