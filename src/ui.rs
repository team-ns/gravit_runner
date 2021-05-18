use druid::widget::{FillStrat, Flex, Image, Label, Painter, ProgressBar};
use druid::{
    AppDelegate, Command, Data, DelegateCtx, Env, Handled, ImageBuf, Lens, Selector, Target,
    Widget, WidgetExt,
};

pub const UPDATE_STAGE: Selector<Status> = Selector::new("event.update-stage");

pub enum Status {
    RunLauncher,
    VerifyLauncher,
    DownloadLauncher,
    DownloadJre,
    CheckJreArchive,
    ExtractJre,
    CheckJreFolder,
}

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub stage: f64,
    pub stage_text: String,
}

pub struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        if let Some(stage) = cmd.get(UPDATE_STAGE) {
            match stage {
                Status::RunLauncher => {
                    data.stage = 1.;
                    data.stage_text = "Run Launcher".to_string();
                }
                Status::VerifyLauncher => {
                    data.stage = 0.75;
                    data.stage_text = "Verify Launcher".to_string();
                }
                Status::DownloadLauncher => {
                    data.stage = 0.60;
                    data.stage_text = "Download Launcher".to_string();
                }
                Status::DownloadJre => {
                    data.stage = 0.;
                    data.stage_text = "Download JRE".to_string();
                }
                Status::CheckJreArchive => {
                    data.stage = 0.15;
                    data.stage_text = "Verify JRE Archive".to_string();
                }
                Status::ExtractJre => {
                    data.stage = 0.30;
                    data.stage_text = "Extract JRE Archive".to_string();
                }
                Status::CheckJreFolder => {
                    data.stage = 0.45;
                    data.stage_text = "Verify JRE Folder".to_string();
                }
            }
            Handled::Yes
        } else {
            Handled::No
        }
    }
}

pub fn build_root_widget() -> impl Widget<AppState> {
    let blurb = Label::new(|data: &AppState, _env: &_| format!("{}", data.stage_text));
    let buf = ImageBuf::from_data(include_bytes!("../background.png")).unwrap();
    let mut image_widget = Image::new(buf).fill_mode(FillStrat::Cover);
    Flex::column()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::Center)
        .must_fill_main_axis(true)
        .with_spacer(10.0)
        .with_child(
            ProgressBar::new()
                .expand_width()
                .padding(10.0)
                .lens(AppState::stage),
        )
        .with_spacer(0.0)
        .with_child(blurb)
        .background(Painter::new(move |ctx, data, env| {
            image_widget.paint(ctx, data, env)
        }))
}
