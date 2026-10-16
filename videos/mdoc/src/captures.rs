use fframes::{EncoderOptions, RenderOptions, StaticMediaProvider, cli};
use fframes_skia_renderer::{
    SkiaFFramesRenderer, SkiaPipelineConcurrencyPolicy, SkiaPipelineConfig, metal::SkiaMetalCtx,
};
mod current;
use current::CaptureVideo;
use fframes::include_media_dir;
include_media_dir!(pub struct CaptureMedia, "media");
const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
use fframes::cli::clap; // the derive below expands to `clap::...`
use std::process::ExitCode;

/// Flags of this video next to the standard ones of `fframes::cli` (render, frame, strip,
/// inspect, audio, ...). Run `cargo run --release -- --help`.
#[derive(Debug, clap::Args)]
struct VideoArgs {
    /// Actual app film: review, compare, share, or current (all three).
    #[arg(long, default_value = "current", global = true)]
    cut: String,
}

fn main() -> ExitCode {
    let args = cli::parse::<VideoArgs>();
    let media = CaptureMedia::prepare().expect("media");
    if !["review", "compare", "share", "current"].contains(&args.app.cut.as_str()) {
        eprintln!("--cut must be review, compare, share or current");
        return ExitCode::from(2);
    }
    let video = CaptureVideo::new(args.app.cut.clone());
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
