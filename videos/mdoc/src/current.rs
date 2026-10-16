//! Product films from unretouched captures of the installed mdoc beta.10.
use fframes::{
    AudioMap, Color, Duration, FFramesContext, Frame, Scene, Scenes, Svgr, Transform, Video,
    animation::Easing,
};
#[derive(Debug)]
pub struct CaptureScene {
    pub kind: &'static str,
}
impl Scene for CaptureScene {
    fn name(&self) -> &'static str {
        self.kind
    }
    fn duration(&self) -> Duration<'_> {
        Duration::Seconds(7.0)
    }
    fn render_frame<'a>(&'a self, frame: Frame, ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let (number, line1, line2, body1, body2, first, second) = match self.kind {
            "Review" => (
                "01 / REVIEW",
                "Review",
                "identities.",
                "One identity.",
                "Every mention.",
                "review-light.jpg",
                "review-dark.jpg",
            ),
            "Compare" => (
                "02 / COMPARE",
                "Keep the",
                "meaning.",
                "Original wording.",
                "Consistent aliases.",
                "compare-dark.jpg",
                "compare-light.jpg",
            ),
            _ => (
                "03 / HANDOFF",
                "Ready for",
                "your AI.",
                "Review the text.",
                "Copy for AI.",
                "share-light.jpg",
                "share-dark.jpg",
            ),
        };
        let blend = frame
            .animate(&fframes::timeline!(at 3.0 => 3.7, animate 0.0_f32 => 1.0, Easing::EaseInOut));
        let rise = frame.animate(&fframes::timeline!(at 0.0 => 0.65, animate 24.0_f32 => 0.0, Easing::CubicBezier(0.16,1.0,0.3,1.0)));
        let shot = |name, opacity| {
            ctx.get_image(name).map(|image| fframes::svgr!(
                <image href={image.href()} x="600" y="176" width="1160" height="802.51" opacity={opacity} />
            )).unwrap_or_else(Svgr::empty)
        };
        fframes::svgr!(
            <g>
                <rect width="1920" height="1080" fill="#09090b" />
                <text x="160" y="128" font-family="DM Sans" font-size="36" font-weight="700" fill="#fafafa">"mdoc"</text>
                <text x="1760" y="124" text-anchor="end" font-family="DM Sans" font-size="28" fill="#a1a1aa">"ON YOUR COMPUTER"</text>
                <g transform={Transform::translate(0.0, rise)}>
                    <text x="160" y="310" font-family="DM Sans" font-size="28" font-weight="500" fill="#99c7b8">{number}</text>
                    <text x="160" y="414" font-family="DM Sans" font-size="76" font-weight="700" letter-spacing="-2" fill="#fafafa">{line1}</text>
                    <text x="160" y="504" font-family="DM Sans" font-size="76" font-weight="700" letter-spacing="-2" fill="#fafafa">{line2}</text>
                    <text x="160" y="604" font-family="DM Sans" font-size="32" fill="#a1a1aa">{body1}</text>
                    <text x="160" y="650" font-family="DM Sans" font-size="32" fill="#a1a1aa">{body2}</text>
                </g>
                <rect x="604" y="190" width="1152" height="802" rx="14" fill="#000000" opacity="0.25" />
                {shot(first, 1.0)}
                {shot(second, blend)}
                <text x="160" y="886" font-family="DM Sans" font-size="28" fill="#a1a1aa">"Actual app captures."</text>
                <text x="160" y="928" font-family="DM Sans" font-size="28" fill="#99c7b8">"Review before sharing."</text>
            </g>
        )
    }
}
#[derive(Debug)]
pub struct CaptureVideo {
    review: CaptureScene,
    compare: CaptureScene,
    share: CaptureScene,
    cut: String,
}
impl CaptureVideo {
    pub fn new(cut: String) -> Self {
        Self {
            review: CaptureScene { kind: "Review" },
            compare: CaptureScene { kind: "Compare" },
            share: CaptureScene { kind: "Share" },
            cut,
        }
    }
}
impl Video for CaptureVideo {
    const FPS: usize = 30;
    const WIDTH: usize = 1920;
    const HEIGHT: usize = 1080;
    const BACKGROUND_COLOR: Color = Color::BLACK;
    fn duration(&self) -> Duration<'_> {
        Duration::Auto
    }
    fn audio(&self) -> AudioMap<'_> {
        AudioMap::none()
    }
    fn define_scenes(&self) -> Scenes<'_> {
        match self.cut.as_str() {
            "review" => Scenes::from(vec![&self.review as &dyn Scene]),
            "compare" => Scenes::from(vec![&self.compare as &dyn Scene]),
            "share" => Scenes::from(vec![&self.share as &dyn Scene]),
            _ => Scenes::from(vec![&self.review as &dyn Scene, &self.compare, &self.share]),
        }
    }
    fn render_frame<'a>(&'a self, frame: Frame, ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        fframes::svgr!(<svg xmlns="http://www.w3.org/2000/svg" width="1920" height="1080">{ctx.render_scenes(&frame)}</svg>)
    }
}
