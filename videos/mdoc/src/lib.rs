//! Explainer videos for the mdoc product page (lawkitt.com/mdoc), in the site's
//! dark style: DM Sans, JetBrains Mono, near-black background, blue accent,
//! amber for detected personal data.
//!
//! One project, several cuts (`--cut ocr|pseudo|ai|hero|readme`). Standalone cuts fade
//! in from and out to the background so they loop as muted autoplay.
use fframes::{
    AnimateRuntimeInput, AudioMap, Color, Duration, FFramesContext, FontQuery, Frame, Overlap,
    Scene, Scenes, Svgr, TextOverflow, Transform, Video,
    animation::{AnimationRuntime, Easing},
    include_media_dir,
};
use std::sync::LazyLock;

include_media_dir!(pub struct MdocVideosMedia, "media");

pub const WIDTH: usize = 1920;
pub const HEIGHT: usize = 1080;

// lawkitt.com palette.
const BG: &str = "#09090b";
const CARD: &str = "#111113";
const ELEV: &str = "#18181b";
const BORDER: &str = "#ffffff17";
const BORDER_STRONG: &str = "#ffffff29";
const FG: &str = "#fafafa";
const MUTED: &str = "#a1a1aa";
const DIM: &str = "#898993";
const ACCENT: &str = "#319cfc";
const ACCENT_DIM: &str = "#319cfc24";
const AMBER: &str = "#e8cf8f";
const AMBER_BG: &str = "#d3b46b33";
const OK: &str = "#7ee2a8";

const SANS: &str = "DM Sans";
const MONO: &str = "JetBrains Mono";

/// Left edge and right edge of the content grid (about 8% margins).
const LEFT: f32 = 160.0;
const RIGHT: f32 = 1760.0;

static EASE: LazyLock<AnimationRuntime> =
    LazyLock::new(|| AnimationRuntime::new(0.6, &Easing::CubicBezier(0.16, 1.0, 0.3, 1.0)));
static EASE_SLOW: LazyLock<AnimationRuntime> =
    LazyLock::new(|| AnimationRuntime::new(0.9, &Easing::CubicBezier(0.65, 0.0, 0.35, 1.0)));
static FADE_OUT: LazyLock<AnimationRuntime> =
    LazyLock::new(|| AnimationRuntime::new(0.3, &Easing::EaseIn));
static SPRING: LazyLock<AnimationRuntime> = LazyLock::new(|| {
    AnimationRuntime::new(3.0, &Easing::Spring { mass: 1.0, stiffness: 180.0, damping: 22.0 })
});

/// 0 → 1 with a fast-then-settle ease, starting at `start` seconds.
fn ramp(frame: &Frame, start: f32) -> f32 {
    frame.animate_runtime(AnimateRuntimeInput { on_second: start, from: 0.0, to: 1.0, animation_runtime: &EASE })
}
/// 0 → 1 with an ease-in-out, for morphs and sweeps.
fn morph(frame: &Frame, start: f32) -> f32 {
    frame.animate_runtime(AnimateRuntimeInput { on_second: start, from: 0.0, to: 1.0, animation_runtime: &EASE_SLOW })
}
/// Springs from `from` px to 0 at `start`.
fn rise(frame: &Frame, start: f32, from: f32) -> f32 {
    frame.animate_runtime(AnimateRuntimeInput { on_second: start, from, to: 0.0, animation_runtime: &SPRING })
}
/// Scene-level opacity: in over 0.4 s, out over the last 0.3 s.
fn scene_fade(frame: &Frame, seconds: f32) -> f32 {
    let fade_in = ramp(frame, 0.0);
    let fade_out = frame.animate_runtime(AnimateRuntimeInput {
        on_second: seconds - 0.3,
        from: 1.0,
        to: 0.0,
        animation_runtime: &FADE_OUT,
    });
    fade_in * fade_out
}
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn width_of<'a>(frame: &mut Frame, ctx: &FFramesContext<'a, '_>, family: &'a str, size: usize, weight: u16, text: &'a str) -> f32 {
    let font = FontQuery { family, size, weight, ..Default::default() };
    frame.text_width(ctx, font, text).unwrap_or(text.len() * size / 2) as f32
}

/// Eyebrow label and title shared by every scene.
fn heading<'a>(frame: &Frame, eyebrow: &'a str, title: &'a str) -> Svgr<'a> {
    let o1 = ramp(frame, 0.15);
    let o2 = ramp(frame, 0.3);
    fframes::svgr!(<g font-family={SANS}>
        <g opacity={o1} transform={Transform::translate(0.0, rise(frame, 0.15, 24.0))}>
            <text x={LEFT} y="150" font-size="28" font-weight="500" letter-spacing="4" fill={ACCENT}>{eyebrow}</text>
        </g>
        <g opacity={o2} transform={Transform::translate(0.0, rise(frame, 0.3, 32.0))}>
            <text x={LEFT} y="232" font-size="80" font-weight="700" letter-spacing="-2.5" fill={FG}>{title}</text>
        </g>
    </g>)
}

/// A rounded pill with a check mark, used for closing statements; sized to its text.
fn pill<'a>(frame: &mut Frame, ctx: &FFramesContext<'a, '_>, start: f32, x: f32, y: f32, text: &'a str) -> Svgr<'a> {
    let width = width_of(frame, ctx, SANS, 32, 500, text) + 68.0 + 34.0;
    let o = ramp(frame, start);
    fframes::svgr!(<g opacity={o} transform={Transform::translate(x, y + rise(frame, start, 20.0))}>
        <rect x="0" y="0" width={width} height="68" rx="34" fill={ELEV} stroke={BORDER_STRONG} stroke-width="1.5" />
        <circle cx="38" cy="34" r="14" fill={OK} />
        <path d="M31 34 l5 5 l9 -10" stroke={BG} stroke-width="3.5" fill="none" stroke-linecap="round" stroke-linejoin="round" />
        <text x="68" y="45" font-family={SANS} font-size="32" font-weight="500" fill={FG}>{text}</text>
    </g>)
}

// ---------------------------------------------------------------- video

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cut {
    Ocr,
    Pseudo,
    Ai,
    Hero,
    /// Recognition then pseudonymization, for the README.
    Readme,
}
impl Cut {
    pub fn parse(name: &str) -> Option<Self> {
        match name {
            "ocr" => Some(Self::Ocr),
            "pseudo" => Some(Self::Pseudo),
            "ai" => Some(Self::Ai),
            "hero" => Some(Self::Hero),
            "readme" => Some(Self::Readme),
            _ => None,
        }
    }
}

pub struct MdocVideosVideo<'a> {
    pub media: &'a MdocVideosMedia,
    cut: Cut,
    ocr: OcrScene,
    pseudo: PseudoScene,
    ai: AiScene,
    outro: OutroScene,
}

impl<'a> MdocVideosVideo<'a> {
    pub fn new(media: &'a MdocVideosMedia, cut: Cut) -> Self {
        let chained = matches!(cut, Cut::Hero | Cut::Readme);
        Self {
            media,
            cut,
            ocr: OcrScene,
            pseudo: PseudoScene { chained },
            ai: AiScene { chained },
            outro: OutroScene,
        }
    }
}

impl std::fmt::Debug for MdocVideosVideo<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MdocVideosVideo").field("cut", &self.cut).finish()
    }
}

impl Video for MdocVideosVideo<'_> {
    const FPS: usize = 30;
    const WIDTH: usize = WIDTH;
    const HEIGHT: usize = HEIGHT;
    const BACKGROUND_COLOR: Color = Color::BLACK;

    fn duration(&self) -> Duration<'_> {
        Duration::Auto
    }

    fn audio(&self) -> AudioMap<'_> {
        AudioMap::none()
    }

    fn define_scenes(&self) -> Scenes<'_> {
        match self.cut {
            Cut::Ocr => Scenes::from(vec![&self.ocr as &dyn Scene]),
            Cut::Pseudo => Scenes::from(vec![&self.pseudo as &dyn Scene]),
            Cut::Ai => Scenes::from(vec![&self.ai as &dyn Scene]),
            Cut::Readme => Scenes::from(vec![&self.ocr as &dyn Scene, &self.pseudo]),
            Cut::Hero => Scenes::from(vec![
                &self.ocr as &dyn Scene,
                &self.pseudo,
                &self.ai,
                &self.outro,
            ]),
        }
    }

    fn render_frame<'a>(&'a self, frame: Frame, ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        fframes::svgr!(
            <svg xmlns="http://www.w3.org/2000/svg" width={WIDTH} height={HEIGHT} viewBox="0 0 1920 1080">
                <rect width="1920" height="1080" fill={BG} />
                {ctx.render_scenes(&frame)}
            </svg>
        )
    }
}

// ---------------------------------------------------------------- 1. scan → Markdown

const OCR_SECONDS: f32 = 10.0;
/// The page card: 640 px tall, the scan's aspect ratio (930 × 1400).
const PAGE_X: f32 = LEFT;
const PAGE_Y: f32 = 300.0;
const PAGE_H: f32 = 620.0;
const PAGE_W: f32 = PAGE_H * 930.0 / 1400.0;
const SWEEP_START: f32 = 1.3;
const SWEEP_SECONDS: f32 = 3.6;

/// Markdown lines and where (fraction of the page height) their source sits.
const OCR_LINES: [(&str, f32, bool); 8] = [
    ("# Brown v. Board of Education", 0.07, true),
    ("Syllabus.", 0.12, false),
    ("Brown et al. v. Board of Education of Topeka et al.", 0.19, false),
    ("No. 1. Appeal from the United States District Court", 0.25, false),
    ("for the District of Kansas.", 0.27, false),
    ("Argued December 9, 1952. Decided May 17, 1954.", 0.33, false),
    ("Segregation of white and Negro children in the public", 0.37, false),
    ("schools of a State solely on the basis of race, pursuant", 0.39, false),
];

#[derive(Debug)]
struct OcrScene;

impl OcrScene {
    /// Sweep position as a fraction of the page height.
    fn sweep(frame: &Frame) -> f32 {
        frame.animate_runtime(AnimateRuntimeInput {
            on_second: SWEEP_START,
            from: 0.0,
            to: 1.0,
            animation_runtime: &AnimationRuntime::new(SWEEP_SECONDS, &Easing::CubicBezier(0.45, 0.05, 0.55, 0.95)),
        })
    }
}

impl Scene for OcrScene {
    fn duration(&self) -> Duration<'_> {
        Duration::Seconds(OCR_SECONDS)
    }

    fn render_frame<'a>(&'a self, mut frame: Frame, ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let page = ctx.get_image("scan-page.png").map(|image| image.href());
        let page_in = ramp(&frame, 0.6);
        let sweep = Self::sweep(&frame);
        let scanning = frame.seconds() > SWEEP_START && frame.seconds() < SWEEP_START + SWEEP_SECONDS + 0.2;
        let sweep_y = PAGE_Y + PAGE_H * sweep;
        let scanned_h = (PAGE_H * sweep).max(0.5);

        // Markdown pane to the right of the page.
        let pane_x = PAGE_X + PAGE_W + 80.0;
        let pane_w = RIGHT - pane_x;
        let pane_in = ramp(&frame, 0.85);
        let lines: Vec<Svgr> = OCR_LINES
            .iter()
            .enumerate()
            .map(|(i, (text, at, heading))| {
                // A line lands just after the sweep passes its source.
                let start = SWEEP_START + SWEEP_SECONDS * (at + 0.02);
                let o = ramp(&frame, start);
                let gap = if i >= 6 { 28.0 } else if i >= 5 { 14.0 } else if i >= 1 { 6.0 } else { 0.0 };
                let y = PAGE_Y + 150.0 + i as f32 * 54.0 + gap + if *heading { -6.0 } else { 0.0 };
                let dx = rise(&frame, start, -28.0);
                let (hash, rest) = if *heading { ("# ", &text[2..]) } else { ("", *text) };
                fframes::svgr!(<g opacity={o} transform={Transform::translate(dx, 0.0)}>
                    <text x={pane_x + 40.0} y={y} font-family={MONO} font-size={if *heading { 36 } else { 28 }} fill={if *heading { FG } else { MUTED }}>
                        <tspan fill={ACCENT}>{hash}</tspan>{rest}
                    </text>
                </g>)
            })
            .collect();
        let caret_on = (frame.seconds() * 1.6).fract() < 0.55;
        let last_y = PAGE_Y + 150.0 + 7.0 * 54.0 + 28.0;

        fframes::svgr!(<g opacity={scene_fade(&frame, OCR_SECONDS)}>
            <defs>
                <clipPath id="ocr-page-clip"><rect x={PAGE_X} y={PAGE_Y} width={PAGE_W} height={PAGE_H} rx="14" /></clipPath>
                <linearGradient id="ocr-glow" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="0" stop-color={ACCENT} stop-opacity="0" />
                    <stop offset="1" stop-color={ACCENT} stop-opacity="0.35" />
                </linearGradient>
            </defs>
            {heading(&frame, "TEXT RECOGNITION", "Scans become text you can edit.")}
            // The scanned page.
            <g opacity={page_in} transform={Transform::translate(0.0, rise(&frame, 0.6, 40.0))}>
                <rect x={PAGE_X - 1.5} y={PAGE_Y - 1.5} width={PAGE_W + 3.0} height={PAGE_H + 3.0} rx="15" fill={BORDER_STRONG} />
                <g clip-path="url(#ocr-page-clip)">
                    <rect x={PAGE_X} y={PAGE_Y} width={PAGE_W} height={PAGE_H} fill="#f4f1ea" />
                    {match page {
                        Some(href) => fframes::svgr!(<image href={href} x={PAGE_X} y={PAGE_Y} width={PAGE_W} height={PAGE_H} preserveAspectRatio="xMidYMin slice" />),
                        None => Svgr::empty(),
                    }}
                    // Recognized area tint and the sweep line.
                    <rect x={PAGE_X} y={PAGE_Y} width={PAGE_W} height={scanned_h} fill={ACCENT} opacity="0.07" />
                    <g opacity={if scanning { 1.0 } else { 0.0 }}>
                        <rect x={PAGE_X} y={sweep_y - 90.0} width={PAGE_W} height="90" fill="url(#ocr-glow)" />
                        <rect x={PAGE_X} y={sweep_y - 1.5} width={PAGE_W} height="3" fill={ACCENT} />
                    </g>
                </g>
                <rect x={PAGE_X + 16.0} y={PAGE_Y + PAGE_H - 60.0} width="236" height="44" rx="22" fill={BG} opacity="0.88" />
                <text x={PAGE_X + 134.0} y={PAGE_Y + PAGE_H - 29.0} text-anchor="middle" font-family={SANS} font-size="22" font-weight="500" letter-spacing="2" fill={FG}>"SCANNED PDF"</text>
            </g>
            // The Markdown pane.
            <g opacity={pane_in} transform={Transform::translate(0.0, rise(&frame, 0.85, 40.0))}>
                <rect x={pane_x} y={PAGE_Y} width={pane_w} height={PAGE_H} rx="16" fill={CARD} stroke={BORDER} stroke-width="1.5" />
                <circle cx={pane_x + 36.0} cy={PAGE_Y + 42.0} r="7" fill={BORDER_STRONG} />
                <circle cx={pane_x + 60.0} cy={PAGE_Y + 42.0} r="7" fill={BORDER_STRONG} />
                <circle cx={pane_x + 84.0} cy={PAGE_Y + 42.0} r="7" fill={BORDER_STRONG} />
                <text x={pane_x + 116.0} y={PAGE_Y + 52.0} font-family={MONO} font-size="26" fill={DIM}>"brown-v-board.md"</text>
                <path d={format!("M{} {} H{}", pane_x, PAGE_Y + 84.0, pane_x + pane_w)} stroke={BORDER} stroke-width="1.5" />
                {lines}
                <rect x={pane_x + 40.0} y={last_y + 18.0} width="14" height="4" fill={ACCENT} opacity={if caret_on && frame.seconds() > 5.2 { 1.0 } else { 0.0 }} />
            </g>
            {pill(&mut frame, ctx, 6.0, LEFT, 958.0, "Recognized on your computer. Nothing is uploaded.")}
        </g>)
    }
}

// ---------------------------------------------------------------- 2. pseudonymization

const PSEUDO_SECONDS: f32 = 12.5;

#[derive(Debug, Clone, Copy)]
enum Kind {
    Person,
    Org,
    Email,
    Phone,
}
impl Kind {
    fn label(self) -> &'static str {
        match self {
            Kind::Person => "PERSON",
            Kind::Org => "ORGANIZATION",
            Kind::Email => "EMAIL",
            Kind::Phone => "PHONE",
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Token {
    Text(&'static str),
    /// Original wording, alias, kind, detection order.
    Entity(&'static str, &'static str, Kind, usize),
}

/// A fictitious clause; the phone number is from Ofcom's range reserved for drama.
const CLAUSE: [&[Token]; 4] = [
    &[Token::Text("This Agreement is made with "), Token::Entity("Northbridge Legal Ltd.", "ORG_1", Kind::Org, 0)],
    &[Token::Text("on behalf of "), Token::Entity("Alice Morgan", "PERSON_1", Kind::Person, 1), Token::Text(", the client.")],
    &[Token::Text("Notices to "), Token::Entity("Alice Morgan", "PERSON_1", Kind::Person, 2), Token::Text(" are sent to")],
    &[
        Token::Entity("alice.morgan@example.com", "EMAIL_1", Kind::Email, 3),
        Token::Text(" or "),
        Token::Entity("020 7946 0958", "PHONE_1", Kind::Phone, 4),
        Token::Text("."),
    ],
];
const DETECT_START: f32 = 1.5;
const DETECT_STEP: f32 = 0.3;
const SWAP_START: f32 = 4.4;
const SWAP_STEP: f32 = 0.35;

/// Legend rows: alias, original, uses.
const LEGEND: [(&str, &str, &str, usize); 4] = [
    ("ORG_1", "Northbridge Legal Ltd.", "1×", 0),
    ("PERSON_1", "Alice Morgan", "2×", 1),
    ("EMAIL_1", "alice.morgan@example.com", "1×", 3),
    ("PHONE_1", "020 7946 0958", "1×", 4),
];

#[derive(Debug)]
struct PseudoScene {
    chained: bool,
}

impl Scene for PseudoScene {
    fn duration(&self) -> Duration<'_> {
        Duration::Seconds(PSEUDO_SECONDS)
    }

    fn overlap(&self) -> Overlap {
        if self.chained { Overlap::Previous(0.4) } else { Overlap::None }
    }

    fn render_frame<'a>(&'a self, mut frame: Frame, ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let card_x = LEFT;
        let card_y = 300.0;
        let card_w = 1010.0;
        let card_h = 560.0;
        let inner = card_w - 96.0;

        // Largest body size at which every original line fits the card.
        let mut size = 44usize;
        loop {
            let widest = CLAUSE
                .iter()
                .map(|line| {
                    line.iter()
                        .map(|t| match t {
                            Token::Text(s) => width_of(&mut frame, ctx, SANS, size, 400, s.trim()) + (s.len() - s.trim().len()) as f32 * size as f32 * 0.28,
                            Token::Entity(s, ..) => width_of(&mut frame, ctx, SANS, size, 400, s) + 12.0,
                        })
                        .sum::<f32>()
                })
                .fold(0.0, f32::max);
            if widest <= inner || size <= 30 {
                break;
            }
            size -= 2;
        }
        let chip_size = (size as f32 * 0.78) as usize;
        let space = width_of(&mut frame, ctx, SANS, size, 400, "a a") - width_of(&mut frame, ctx, SANS, size, 400, "aa");

        let mut nodes: Vec<Svgr> = Vec::new();
        for (row, line) in CLAUSE.iter().enumerate() {
            let baseline = card_y + 160.0 + row as f32 * 92.0;
            let mut x = card_x + 48.0;
            for token in line.iter() {
                match *token {
                    Token::Text(s) => {
                        let lead = s.len() - s.trim_start().len();
                        let trail = s.len() - s.trim_end().len();
                        let w = width_of(&mut frame, ctx, SANS, size, 400, s.trim()) + (lead + trail) as f32 * space;
                        nodes.push(fframes::svgr!(<text x={x + lead as f32 * space} y={baseline} font-family={SANS} font-size={size} font-weight="400" fill={FG}>{s.trim()}</text>));
                        x += w;
                    }
                    Token::Entity(original, alias, kind, order) => {
                        let detect = ramp(&frame, DETECT_START + order as f32 * DETECT_STEP);
                        let swap = morph(&frame, SWAP_START + order as f32 * SWAP_STEP);
                        let w_orig = width_of(&mut frame, ctx, SANS, size, 400, original);
                        let w_alias = width_of(&mut frame, ctx, MONO, chip_size, 400, alias) + 28.0;
                        let w = lerp(w_orig, w_alias, swap);
                        let pad = 6.0;
                        let top = baseline - size as f32 * 0.86;
                        let h = size as f32 * 1.16;
                        let label_o = detect * (1.0 - swap);
                        x += pad + 4.0;
                        nodes.push(fframes::svgr!(<g>
                            // Amber detection highlight, turning into the blue alias chip.
                            <rect x={x - pad} y={top} width={(w + pad * 2.0).max(0.5)} height={h} rx="8"
                                fill={AMBER_BG} opacity={detect * (1.0 - swap)} />
                            <rect x={x - pad} y={top + h - 3.0} width={(w + pad * 2.0) * detect + 0.5} height="3" rx="1.5"
                                fill={AMBER} opacity={1.0 - swap} />
                            <rect x={x - pad} y={top} width={(w + pad * 2.0).max(0.5)} height={h} rx="8"
                                fill={ACCENT_DIM} stroke={ACCENT} stroke-opacity="0.45" stroke-width="1.5" opacity={swap} />
                            <text x={x} y={top - 10.0} font-family={SANS} font-size="20" font-weight="500" letter-spacing="2" fill={AMBER} opacity={label_o}>{kind.label()}</text>
                            <g opacity={1.0 - swap}>
                                <text x={x} y={baseline} font-family={SANS} font-size={size} font-weight="400" fill={FG}>{original}</text>
                            </g>
                            <g opacity={swap}>
                                <text x={x + 14.0 * swap} y={baseline - 2.0} font-family={MONO} font-size={chip_size} fill={ACCENT}>{alias}</text>
                            </g>
                        </g>));
                        x += w + pad + 4.0;
                    }
                }
            }
        }

        // Status line under the clause.
        let found = ramp(&frame, DETECT_START + 5.0 * DETECT_STEP + 0.1) * (1.0 - morph(&frame, SWAP_START - 0.3));
        let applied = ramp(&frame, SWAP_START + 5.0 * SWAP_STEP);

        // The lawyer clicks "Apply replacements": a pointer glides to the button and presses it.
        let btn_w = 290.0;
        let btn_h = 60.0;
        let btn_x = card_x + card_w - 48.0 - btn_w;
        let btn_y = card_y + card_h - 44.0 - 40.0;
        let press_at = SWAP_START - 0.45;
        let press = ramp(&frame, press_at) * (1.0 - ramp(&frame, press_at + 0.2));
        let btn_scale = 1.0 - 0.06 * press;
        let pressed = ramp(&frame, press_at);
        let glide = morph(&frame, press_at - 1.1);
        let pointer_x = lerp(btn_x + btn_w + 260.0, btn_x + btn_w * 0.62, glide);
        let pointer_y = lerp(btn_y + 230.0, btn_y + btn_h * 0.55, glide);
        let pointer_o = ramp(&frame, press_at - 1.2) * (1.0 - ramp(&frame, press_at + 0.5));

        // Replacement legend on the right.
        let legend_x = card_x + card_w + 48.0;
        let legend_w = RIGHT - legend_x;
        let rows: Vec<Svgr> = LEGEND
            .iter()
            .enumerate()
            .map(|(i, (alias, original, uses, order))| {
                let start = SWAP_START + *order as f32 * SWAP_STEP + 0.25;
                let o = ramp(&frame, start);
                let y = card_y + 140.0 + i as f32 * 98.0;
                let original = frame
                    .text_fit(ctx, FontQuery { family: SANS, size: 26, weight: 400, ..Default::default() }, original, (legend_w - 140.0) as usize, TextOverflow::Ellipsis)
                    .map(|t| t.into_owned())
                    .unwrap_or_else(|| original.to_string());
                fframes::svgr!(<g opacity={o} transform={Transform::translate(rise(&frame, start, 30.0), 0.0)}>
                    <text x={legend_x + 36.0} y={y} font-family={MONO} font-size="28" fill={ACCENT}>{*alias}</text>
                    <text x={legend_x + legend_w - 36.0} y={y} text-anchor="end" font-family={SANS} font-size="26" font-weight="500" fill={DIM}>{*uses}</text>
                    <text x={legend_x + 36.0} y={y + 36.0} font-family={SANS} font-size="26" fill={MUTED}>{original}</text>
                </g>)
            })
            .collect();
        let legend_in = ramp(&frame, SWAP_START);

        fframes::svgr!(<g opacity={scene_fade(&frame, PSEUDO_SECONDS)}>
            {heading(&frame, "PSEUDONYMIZATION", "Client names become consistent aliases.")}
            <g opacity={ramp(&frame, 0.5)} transform={Transform::translate(0.0, rise(&frame, 0.5, 40.0))}>
                <rect x={card_x} y={card_y} width={card_w} height={card_h} rx="16" fill={CARD} stroke={BORDER} stroke-width="1.5" />
                <text x={card_x + 48.0} y={card_y + 64.0} font-family={SANS} font-size="26" font-weight="500" letter-spacing="2" fill={DIM}>"SERVICES AGREEMENT · CLAUSE 1"</text>
                {nodes}
                <g opacity={found}>
                    <text x={card_x + 48.0} y={card_y + card_h - 44.0} font-family={SANS} font-size="28" font-weight="500" fill={AMBER}>"5 proposals found locally · review them"</text>
                    <g transform={format!("translate({} {}) scale({btn_scale}) translate({} {})", btn_x + btn_w / 2.0, btn_y + btn_h / 2.0, -(btn_x + btn_w / 2.0), -(btn_y + btn_h / 2.0))}>
                        <rect x={btn_x} y={btn_y} width={btn_w} height={btn_h} rx="12" fill={ACCENT} opacity={0.18 + 0.2 * pressed} stroke={ACCENT} stroke-width="1.5" />
                        <text x={btn_x + btn_w / 2.0} y={btn_y + 40.0} text-anchor="middle" font-family={SANS} font-size="28" font-weight="500" fill={FG}>"Apply replacements"</text>
                    </g>
                </g>
                <g opacity={applied}>
                    <text x={card_x + 48.0} y={card_y + card_h - 44.0} font-family={SANS} font-size="28" font-weight="500" fill={OK}>"Applied · the same person always gets the same alias"</text>
                </g>
            </g>
            <g opacity={legend_in} transform={Transform::translate(0.0, rise(&frame, SWAP_START, 30.0))}>
                <rect x={legend_x} y={card_y} width={legend_w} height={card_h} rx="16" fill={CARD} stroke={BORDER} stroke-width="1.5" />
                <text x={legend_x + 36.0} y={card_y + 64.0} font-family={SANS} font-size="26" font-weight="500" letter-spacing="2" fill={DIM}>"REPLACEMENTS"</text>
                {rows}
            </g>
            {pill(&mut frame, ctx, 7.6, LEFT, 958.0, "You approve every replacement. Originals stay on this computer.")}
            // Pointer arrow, drawn last so it sits above the card.
            <g opacity={pointer_o} transform={Transform::translate(pointer_x, pointer_y)}>
                <path d="M0 0 L0 34 L9 26 L15 40 L21 37 L15 24 L27 24 Z" fill={FG} stroke={BG} stroke-width="2" stroke-linejoin="round" />
            </g>
        </g>)
    }
}

// ---------------------------------------------------------------- 3. handoff to an AI tool

const AI_SECONDS: f32 = 9.5;
const PREPARED: [&str; 4] = [
    "This Agreement is made with ORG_1",
    "on behalf of PERSON_1, the client.",
    "Notices to PERSON_1 are sent to",
    "EMAIL_1 or PHONE_1.",
];

#[derive(Debug)]
struct AiScene {
    chained: bool,
}

/// A line of prepared Markdown with its aliases drawn as accent text.
fn alias_line<'a>(x: f32, y: f32, family: &'a str, size: usize, base: &'a str, line: &'a str) -> Svgr<'a> {
    let parts: Vec<Svgr> = line
        .split_inclusive(' ')
        .map(|word| {
            let alias = word.trim_end_matches([' ', ',', '.']).contains('_');
            fframes::svgr!(<tspan fill={if alias { ACCENT } else { base }}>{word}</tspan>)
        })
        .collect();
    fframes::svgr!(<text x={x} y={y} font-family={family} font-size={size}>{parts}</text>)
}

impl Scene for AiScene {
    fn duration(&self) -> Duration<'_> {
        Duration::Seconds(AI_SECONDS)
    }

    fn overlap(&self) -> Overlap {
        if self.chained { Overlap::Previous(0.4) } else { Overlap::None }
    }

    fn render_frame<'a>(&'a self, mut frame: Frame, ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let doc_x = LEFT;
        let doc_y = 320.0;
        let doc_w = 700.0;
        let doc_h = 420.0;
        let lines: Vec<Svgr> = PREPARED
            .iter()
            .enumerate()
            .map(|(i, line)| alias_line(doc_x + 40.0, doc_y + 150.0 + i as f32 * 54.0, MONO, 26, MUTED, line))
            .collect();

        // Copy button press at 1.6 s.
        let press = ramp(&frame, 1.6);
        let release = ramp(&frame, 1.85);
        let scale = 1.0 - 0.06 * press * (1.0 - release);
        let copied = ramp(&frame, 1.9);
        let btn_x = doc_x + doc_w - 190.0;
        let btn_y = doc_y + 26.0;

        // The text travels from the document into the chat input.
        let travel = morph(&frame, 2.4);
        let chat_x = 980.0;
        let chat_w = RIGHT - chat_x;
        let chat_y = 300.0;
        let chat_h = 600.0;
        let ghost_x = lerp(btn_x - 150.0, chat_x + 56.0, travel);
        let ghost_y = lerp(btn_y + 110.0, chat_y + chat_h - 44.0, travel);
        let ghost_o = if travel > 0.0 && travel < 1.0 { 0.9 } else { 0.0 };
        let sent = ramp(&frame, 3.4);
        let typing = ramp(&frame, 4.4) * (1.0 - ramp(&frame, 5.6));
        let reply = ramp(&frame, 5.7);
        let dots: Vec<Svgr> = (0..3)
            .map(|i| {
                let bob = ((frame.seconds() * 5.0 - i as f32 * 0.6).sin() * 5.0).min(0.0);
                fframes::svgr!(<circle cx={chat_x + 64.0 + i as f32 * 22.0} cy={chat_y + 420.0 + bob} r="6" fill={MUTED} />)
            })
            .collect();
        let bubble: Vec<Svgr> = PREPARED
            .iter()
            .enumerate()
            .map(|(i, line)| alias_line(chat_x + 236.0, chat_y + 150.0 + i as f32 * 40.0, MONO, 22, MUTED, line))
            .collect();

        fframes::svgr!(<g opacity={scene_fade(&frame, AI_SECONDS)}>
            {heading(&frame, "HANDOFF", "Paste prepared text into your AI tool.")}
            // Prepared Markdown with a Copy button.
            <g opacity={ramp(&frame, 0.5)} transform={Transform::translate(0.0, rise(&frame, 0.5, 40.0))}>
                <rect x={doc_x} y={doc_y} width={doc_w} height={doc_h} rx="16" fill={CARD} stroke={BORDER} stroke-width="1.5" />
                <text x={doc_x + 40.0} y={doc_y + 72.0} font-family={MONO} font-size="26" fill={DIM}>"agreement.md"</text>
                {lines}
                <g transform={format!("translate({} {}) scale({scale}) translate({} {})", btn_x + 75.0, btn_y + 28.0, -(btn_x + 75.0), -(btn_y + 28.0))}>
                    <rect x={btn_x} y={btn_y} width="150" height="56" rx="12" fill={ELEV} stroke={BORDER_STRONG} stroke-width="1.5" />
                    <rect x={btn_x} y={btn_y} width="150" height="56" rx="12" fill={ACCENT_DIM} stroke={ACCENT} stroke-width="1.5" opacity={copied} />
                    <text x={btn_x + 75.0} y={btn_y + 37.0} text-anchor="middle" font-family={SANS} font-size="26" font-weight="500" fill={FG} opacity={1.0 - copied}>"Copy"</text>
                    <text x={btn_x + 75.0} y={btn_y + 37.0} text-anchor="middle" font-family={SANS} font-size="26" font-weight="500" fill={ACCENT} opacity={copied}>"Copied"</text>
                </g>
            </g>
            // A generic chat window.
            <g opacity={ramp(&frame, 0.8)} transform={Transform::translate(0.0, rise(&frame, 0.8, 40.0))}>
                <rect x={chat_x} y={chat_y} width={chat_w} height={chat_h} rx="16" fill={CARD} stroke={BORDER} stroke-width="1.5" />
                <circle cx={chat_x + 36.0} cy={chat_y + 42.0} r="7" fill={BORDER_STRONG} />
                <circle cx={chat_x + 60.0} cy={chat_y + 42.0} r="7" fill={BORDER_STRONG} />
                <circle cx={chat_x + 84.0} cy={chat_y + 42.0} r="7" fill={BORDER_STRONG} />
                <text x={chat_x + 116.0} y={chat_y + 52.0} font-family={SANS} font-size="26" font-weight="500" fill={DIM}>"Your AI assistant"</text>
                <path d={format!("M{} {} H{}", chat_x, chat_y + 84.0, chat_x + chat_w)} stroke={BORDER} stroke-width="1.5" />
                // The user's message.
                <g opacity={sent} transform={Transform::translate(0.0, rise(&frame, 3.4, 24.0))}>
                    <rect x={chat_x + 206.0} y={chat_y + 108.0} width={chat_w - 236.0} height="196" rx="18" fill={ELEV} stroke={BORDER_STRONG} stroke-width="1.5" />
                    {bubble}
                </g>
                // Typing, then a reply that only knows the aliases.
                <g opacity={typing}>{dots}</g>
                <g opacity={reply} transform={Transform::translate(0.0, rise(&frame, 5.7, 24.0))}>
                    {alias_line(chat_x + 40.0, chat_y + 380.0, SANS, 26, FG, "ORG_1 acts for PERSON_1. Notices go to")}
                    {alias_line(chat_x + 40.0, chat_y + 420.0, SANS, 26, FG, "EMAIL_1 or PHONE_1. Key obligations:")}
                    <rect x={chat_x + 40.0} y={chat_y + 450.0} width="520" height="14" rx="7" fill={BORDER_STRONG} />
                    <rect x={chat_x + 40.0} y={chat_y + 480.0} width="430" height="14" rx="7" fill={BORDER_STRONG} />
                </g>
                // Input field.
                <rect x={chat_x + 24.0} y={chat_y + chat_h - 84.0} width={chat_w - 48.0} height="60" rx="30" fill={ELEV} stroke={BORDER} stroke-width="1.5" />
                <text x={chat_x + 56.0} y={chat_y + chat_h - 44.0} font-family={SANS} font-size="24" fill={DIM} opacity={1.0 - sent}>"Paste text…"</text>
            </g>
            // The copied text in flight.
            <g opacity={ghost_o}>
                <rect x={ghost_x - 20.0} y={ghost_y - 36.0} width="330" height="56" rx="28" fill={CARD} stroke={ACCENT} stroke-width="1.5" />
                <text x={ghost_x} y={ghost_y} font-family={SANS} font-size="24" font-weight="500" fill={ACCENT}>"Prepared Markdown"</text>
            </g>
            {pill(&mut frame, ctx, 6.6, LEFT, 958.0, "The AI tool sees aliases, never your client's names.")}
        </g>)
    }
}

// ---------------------------------------------------------------- outro (hero only)

const OUTRO_SECONDS: f32 = 4.0;

#[derive(Debug)]
struct OutroScene;

impl Scene for OutroScene {
    fn duration(&self) -> Duration<'_> {
        Duration::Seconds(OUTRO_SECONDS)
    }

    fn overlap(&self) -> Overlap {
        Overlap::Previous(0.4)
    }

    fn render_frame<'a>(&'a self, frame: Frame, ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let icon = ctx.get_image("mdoc-icon.png").map(|i| i.href());
        let o = scene_fade(&frame, OUTRO_SECONDS);
        let icon_in = ramp(&frame, 0.2);
        let title_in = ramp(&frame, 0.4);
        let line_in = ramp(&frame, 0.6);
        let scale = lerp(0.92, 1.0, icon_in);
        fframes::svgr!(<g opacity={o}>
            <g opacity={icon_in} transform={format!("translate(960 420) scale({scale}) translate(-80 -80)")}>
                <defs><clipPath id="outro-icon-clip"><rect x="10" y="10" width="140" height="140" rx="34" /></clipPath></defs>
                <g clip-path="url(#outro-icon-clip)">
                    {match icon {
                        Some(href) => fframes::svgr!(<image href={href} x="0" y="0" width="160" height="160" />),
                        None => Svgr::empty(),
                    }}
                </g>
            </g>
            <g opacity={title_in} transform={Transform::translate(0.0, rise(&frame, 0.4, 24.0))}>
                <text x="960" y="600" text-anchor="middle" font-family={SANS} font-size="96" font-weight="700" letter-spacing="-3" fill={FG}>"mdoc"</text>
            </g>
            <g opacity={line_in} transform={Transform::translate(0.0, rise(&frame, 0.6, 24.0))}>
                <text x="960" y="676" text-anchor="middle" font-family={SANS} font-size="40" fill={MUTED}>"Prepare legal documents for AI, on your own computer."</text>
                <text x="960" y="748" text-anchor="middle" font-family={SANS} font-size="30" font-weight="500" letter-spacing="2" fill={ACCENT}>"FREE · OPEN SOURCE · MACOS AND WINDOWS"</text>
            </g>
        </g>)
    }
}
