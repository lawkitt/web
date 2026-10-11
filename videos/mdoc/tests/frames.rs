//! Visual regression: renders a few frames and compares them with the approved PNGs in
//! `_frame_snapshots/`. The first run only creates them, so look at the PNGs before you
//! commit them. `FFRAMES_UPDATE_SNAPSHOTS=1 cargo test` accepts intentional changes; failing
//! frames leave `.actual.png` and `.diff.png` files.
use fframes::{CpuFrameRenderer, Previewer, RenderOptions, StaticMediaProvider, snapshot};
use mdoc_videos::{Cut, MdocVideosMedia, MdocVideosVideo};

#[test]
fn key_frames_match_snapshots() {
    let media = MdocVideosMedia::prepare().unwrap();
    let video = MdocVideosVideo::new(&media, Cut::Hero);
    let options = RenderOptions {
        media: Some(&media),
        scale_resolution: 0.5,
        ..Default::default()
    };
    let mut previewer = Previewer::new(&video, &options).unwrap();

    snapshot::assert_frames(
        &mut previewer,
        &mut CpuFrameRenderer::default(),
        // Settled frames: the middle of a scene, not the end of it where it fades out.
        &["OcrScene@8s", "PseudoScene@4s", "PseudoScene@10s", "AiScene@7s", "OutroScene@2s"],
        &snapshot::SnapshotOptions::default(),
    );
}

/// Every cut, not just the hero: standalone cuts fade differently.
#[test]
fn no_problems_in_any_frame() {
    for cut in ["ocr", "pseudo", "ai", "hero", "readme"] {
        check_cut(Cut::parse(cut).unwrap());
    }
}

fn check_cut(cut: Cut) {
    // Converting a frame without rasterizing it is fast, so every frame is checked.
    let media = MdocVideosMedia::prepare().unwrap();
    let video = MdocVideosVideo::new(&media, cut);
    let options = RenderOptions {
        media: Some(&media),
        ..Default::default()
    };
    let mut previewer = Previewer::new(&video, &options).unwrap();

    let duration = previewer.timeline().duration_in_frames;
    for frame in 0..duration {
        let report = previewer.inspect(frame).unwrap();
        let problems: Vec<_> = report
            .diagnostics
            .iter()
            .filter(|d| d.severity >= fframes::diagnostics::Severity::Warning)
            .map(|d| d.message.as_str())
            .collect();
        assert!(problems.is_empty(), "{cut:?} frame {frame} ({:.2}s): {problems:?}", report.seconds);
    }
}
