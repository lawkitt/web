use fframes::{EncoderOptions, RenderOptions, cli};
use fframes_skia_renderer::{
    SkiaFFramesRenderer, SkiaPipelineConcurrencyPolicy, SkiaPipelineConfig, metal::SkiaMetalCtx,
};
mod scenes;
use scenes::{Film, IDS, Theme};
const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
use fframes::cli::clap; // the derive below expands to `clap::...`
use std::process::ExitCode;

/// Flags of this video next to the standard ones of `fframes::cli` (render, frame, strip,
/// inspect, audio, ...). Run `cargo run --release -- --help`.
#[derive(Debug, clap::Args)]
struct VideoArgs {
    /// Library scenes to play, comma separated in any order (detect,tooltip,popup), or `all`.
    #[arg(long, default_value = "all", global = true)]
    cut: String,
    /// App theme of the recordings: dark or light.
    #[arg(long, default_value = "dark", global = true)]
    theme: String,
}

fn main() -> ExitCode {
    let args = cli::parse::<VideoArgs>();
    // Runtime media: the compile-time media macro cannot hold the MP4 recordings.
    let dir = fframes::MediaDirectory::read_folder(concat!(env!("CARGO_MANIFEST_DIR"), "/film")).expect("film media");
    let media = dir.process_media_source().expect("film media");
    let cut: Vec<&str> = if args.app.cut == "all" { vec![] } else { args.app.cut.split(',').collect() };
    if let Some(bad) = cut.iter().find(|id| !IDS.contains(id)) {
        eprintln!("unknown scene {bad}; scenes: {}", IDS.join(", "));
        return ExitCode::from(2);
    }
    let theme = match args.app.theme.as_str() {
        "dark" => Theme::Dark,
        "light" => Theme::Light,
        _ => {
            eprintln!("--theme must be dark or light");
            return ExitCode::from(2);
        }
    };
    let video = Film::new(theme, &cut);
    let gpu = SkiaMetalCtx::new(WIDTH, HEIGHT).expect("GPU context");

    cli::new(
        &video,
        RenderOptions {
            media: Some(&media),
            video_encoder_options: EncoderOptions {
                preferred_encoder: Some("libx264"),
                codec_params: Some(&[("crf", "20"), ("preset", "medium"), ("tune", "animation")]),
                ..Default::default()
            },
            ..Default::default()
        },
    )
    .args(args)
    // Frame previews (frame, strip, onion, snapshot) render with the same Skia backend.
    .backend(
        SkiaFFramesRenderer::new_metal(
            &gpu,
            SkiaPipelineConfig {
                concurrency_policy: SkiaPipelineConcurrencyPolicy::MaxPerformance,
                ..Default::default()
            },
        )
        .expect("skia renderer"),
    )
    // `preview` opens the real-time GPU player.
    .preview(fframes_native_player::cli_preview)
    .run()
}
