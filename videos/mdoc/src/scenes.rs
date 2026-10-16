//! The mdoc scene library: short scenes cut from scripted screen recordings of mdoc
//! 0.1.0-beta.11 (capture/take.sh), one idea each, rendered as separate cuts so the
//! page can pick and reorder them (ADR 0007). Window corners are clipped here, so the
//! recordings' desktop corners never show.
use fframes::{
    AudioMap, Color, Duration, FFramesContext, FFramesSyncedVideoFrame, Frame, Overlap, Scene, Scenes, Svgr,
    SyncVideoFrameInput, Transform, Video, animation::Easing,
};

pub const WIDTH: usize = 1920;
pub const HEIGHT: usize = 1080;
// The 1440 × 900 pt app window, drawn 1:1 below the caption band.
const WIN_X: f32 = 240.0;
const WIN_Y: f32 = 150.0;
const WIN_W: f32 = 1440.0;
const WIN_H: f32 = 900.0;
const WIN_R: f32 = 13.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Theme {
    Dark,
    Light,
}

struct Palette {
    bg: &'static str,
    fg: &'static str,
    muted: &'static str,
    accent: &'static str,
    edge: &'static str,
    surface: &'static str,
    alias: &'static str,
}

impl Theme {
    pub fn name(self) -> &'static str {
        match self {
            Theme::Dark => "dark",
            Theme::Light => "light",
        }
    }
    fn palette(self) -> Palette {
        match self {
            Theme::Dark => Palette {
                bg: "#09090b",
                fg: "#fafafa",
                muted: "#a1a1aa",
                accent: "#99c7b8",
                edge: "#3f3f46",
                surface: "#18181b",
                alias: "#8ab4f8",
            },
            Theme::Light => Palette {
                bg: "#f4f4f5",
                fg: "#18181b",
                muted: "#52525b",
                accent: "#3f7f6c",
                edge: "#d4d4d8",
                surface: "#ffffff",
                alias: "#2563eb",
            },
        }
    }
}

#[derive(Debug)]
enum Kind {
    Intro,
    Still,
    Clip,
    Chat,
}

#[derive(Debug)]
pub struct Shot {
    pub id: &'static str,
    title: &'static str,
    sub: &'static str,
    kind: Kind,
    seconds: f32,
    theme: Theme,
    media: String,
    /// Cross-fades from the previous scene; off for the first scene and single cuts.
    blend: bool,
    /// The next scene cross-fades over this one's end, so the caption leaves first.
    handoff: bool,
}

const BLEND: f32 = 0.3;

impl Scene for Shot {
    fn name(&self) -> &'static str {
        self.id
    }
    fn duration(&self) -> Duration<'_> {
        // The overlap is added on top, so a blended clip scene still ends inside its footage.
        Duration::Seconds(if self.blend { self.seconds - BLEND } else { self.seconds })
    }
    fn overlap(&self) -> Overlap {
        if self.blend { Overlap::Previous(BLEND) } else { Overlap::None }
    }
    fn render_frame<'a>(&'a self, frame: Frame, ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        let p = self.theme.palette();
        // A film opens on its first frame (no black flash between looping films);
        // later scenes fade in over the previous one, the intro card always does.
        let fade = if self.blend || matches!(self.kind, Kind::Intro) {
            frame.animate(&fframes::timeline!(at 0.0 => BLEND, animate 0.0_f32 => 1.0, Easing::EaseOut))
        } else {
            1.0
        };
        let leave = if self.handoff {
            let end = self.seconds;
            1.0 - frame.animate(&fframes::timeline!(at end - BLEND - 0.15 => end - 0.1, animate 0.0_f32 => 1.0, Easing::EaseIn))
        } else {
            1.0
        };
        if let Kind::Intro = self.kind {
            return self.intro(&frame, ctx, &p, fade * leave);
        }
        let rise = frame.animate(&fframes::timeline!(
            at 0.1 => 0.6, animate 14.0_f32 => 0.0, Easing::CubicBezier(0.16, 1.0, 0.3, 1.0)
        ));
        let body = match self.kind {
            Kind::Clip => {
                let input = SyncVideoFrameInput { start_from: 0.0, looping: false, editor_fallback_image: None };
                frame
                    .get_synced_video_frame(ctx, &self.media, &input)
                    .map(|clip| {
                        let image = clip.into_image();
                        fframes::svgr!(<image href={image.href()} x={WIN_X} y={WIN_Y} width={WIN_W} height={WIN_H} />)
                    })
                    .unwrap_or_else(Svgr::empty)
            }
            Kind::Still => {
                // A slow push-in keeps a still from looking frozen.
                let zoom = frame.animate(&fframes::timeline!(at 0.0 => 3.3, animate 1.0_f32 => 1.025, Easing::EaseInOut));
                let (cx, cy) = (WIN_X + WIN_W * 0.3, WIN_Y + WIN_H * 0.45);
                ctx.get_image(&self.media)
                    .map(|image| fframes::svgr!(
                        <g transform={Transform::translate(cx, cy)}>
                            <g transform={Transform::scale(zoom)}>
                                <g transform={Transform::translate(-cx, -cy)}>
                                    <image href={image.href()} x={WIN_X} y={WIN_Y} width={WIN_W} height={WIN_H} />
                                </g>
                            </g>
                        </g>
                    ))
                    .unwrap_or_else(Svgr::empty)
            }
            Kind::Chat => self.chat(&frame, &p),
            Kind::Intro => Svgr::empty(),
        };
        let logo = ctx.get_image("mdoc-logo.png").map(|i| i.href());
        fframes::svgr!(
            <g opacity={fade}>
                <defs>
                    <clipPath id="window">
                        <rect x={WIN_X} y={WIN_Y} width={WIN_W} height={WIN_H} rx={WIN_R} />
                    </clipPath>
                </defs>
                {logo.map(|href| fframes::svgr!(<image href={href} x="240" y="56" width="56" height="56" />)).unwrap_or_else(Svgr::empty)}
                <g transform={Transform::translate(0.0, rise)} opacity={leave}>
                    <text x="320" y="97" font-family="DM Sans" font-size="40" font-weight="700" letter-spacing="-0.6" fill={p.fg}>{self.title}</text>
                    <text x="1680" y="95" text-anchor="end" font-family="DM Sans" font-size="26" fill={p.muted}>{self.sub}</text>
                </g>
                <rect x={WIN_X - 1.0} y={WIN_Y - 1.0} width={WIN_W + 2.0} height={WIN_H + 2.0} rx={WIN_R + 1.0} fill="none" stroke={p.edge} stroke-width="1" />
                <g clip-path="url(#window)">{body}</g>
            </g>
        )
    }
}

impl Shot {
    fn intro<'a>(&'a self, frame: &Frame, ctx: &FFramesContext<'a, '_>, p: &Palette, fade: f32) -> Svgr<'a> {
        let lift = frame.animate(&fframes::timeline!(
            at 0.0 => 0.8, animate 24.0_f32 => 0.0, Easing::Spring { mass: 1.0, stiffness: 160.0, damping: 20.0 }
        ));
        let tagline = frame.animate(&fframes::timeline!(at 0.5 => 1.0, animate 0.0_f32 => 1.0, Easing::EaseOut));
        let logo = ctx.get_image("mdoc-logo.png").map(|i| i.href());
        fframes::svgr!(
            <g opacity={fade}>
                <g transform={Transform::translate(0.0, lift)}>
                    {logo.map(|href| fframes::svgr!(<image href={href} x="880" y="300" width="160" height="160" />)).unwrap_or_else(Svgr::empty)}
                    <text x="960" y="590" text-anchor="middle" font-family="DM Sans" font-size="104" font-weight="700" letter-spacing="-3" fill={p.fg}>"mdoc"</text>
                </g>
                <text x="960" y="670" text-anchor="middle" font-family="DM Sans" font-size="40" fill={p.muted} opacity={tagline}>{self.title}</text>
                <text x="960" y="724" text-anchor="middle" font-family="DM Sans" font-size="30" fill={p.accent} opacity={tagline}>{self.sub}</text>
            </g>
        )
    }

    /// A neutral, unbranded chat: the question and the answer only use aliases.
    fn chat<'a>(&'a self, frame: &Frame, p: &Palette) -> Svgr<'a> {
        const LINES: [&[(&str, bool)]; 5] = [
            &[("Summary for ", false), ("PERSON_1", true)],
            &[("ORG_2", true), (" claims £22,575.50 from ", false), ("ORG_1", true), (" for a late, too-warm consignment.", false)],
            &[("PERSON_2", true), (" rejected the load on arrival and recorded why.", false)],
            &[("Next: ask ", false), ("PERSON_3", true), (" at ", false), ("ORG_5", true), (" for the rejection report.", false)],
            &[("Payment would go to ", false), ("BANK_1", true), (".", false)],
        ];
        let ask = frame.animate(&fframes::timeline!(at 0.2 => 0.6, animate 0.0_f32 => 1.0, Easing::EaseOut));
        let lines: Vec<Svgr<'a>> = LINES
            .iter()
            .enumerate()
            .map(|(i, line)| {
                let start = 1.0 + i as f32 * 0.45;
                let shown = frame.animate(&fframes::timeline!(at start => start + 0.35, animate 0.0_f32 => 1.0, Easing::EaseOut));
                let size = if i == 0 { 34 } else { 30 };
                let weight = if i == 0 { 700 } else { 400 };
                let spans: Vec<Svgr<'a>> = line
                    .iter()
                    .map(|(text, alias)| {
                        let fill = if *alias { p.alias } else { p.fg };
                        let family = if *alias { "JetBrains Mono" } else { "DM Sans" };
                        fframes::svgr!(<tspan fill={fill} font-family={family}>{*text}</tspan>)
                    })
                    .collect();
                fframes::svgr!(
                    <text x="400" y={480.0 + i as f32 * 62.0} font-family="DM Sans" font-size={size} font-weight={weight} opacity={shown}>{spans}</text>
                )
            })
            .collect();
        fframes::svgr!(
            <g>
                <rect x={WIN_X} y={WIN_Y} width={WIN_W} height={WIN_H} fill={p.surface} />
                <text x="400" y="230" font-family="DM Sans" font-size="24" fill={p.muted}>"Any AI chat"</text>
                <g opacity={ask}>
                    <rect x="820" y="270" width="700" height="96" rx="22" fill={p.edge} opacity="0.55" />
                    <text x="852" y="312" font-family="DM Sans" font-size="26" fill={p.fg}>"Summarise this letter and suggest"</text>
                    <text x="852" y="346" font-family="DM Sans" font-size="26" fill={p.fg}>"next steps for the client."</text>
                </g>
                {lines}
            </g>
        )
    }
}

pub struct Film {
    shots: Vec<Shot>,
}

/// Scene id, caption, sub-caption, content and length (clip lengths match capture/clips.sh).
const LIBRARY: [(&str, &str, &str, &str, f32); 13] = [
    ("intro", "Prepare legal documents for AI.", "On your computer.", "intro", 3.0),
    ("scan", "Scans become text", "Recognition runs on this computer", "still:still-scan", 3.0),
    ("detect", "Find what identifies people", "Names, companies, addresses, bank details", "clip:detect", 6.0),
    ("tooltip", "Every mention is labelled", "Hover to see its alias", "clip:tooltip", 3.4),
    ("popup", "One identity, every mention", "Change it once, everywhere", "clip:popup", 3.0),
    ("alias", "Reuse or rename aliases", "Same as… or a new alias", "clip:alias", 2.5),
    ("keep", "Keep what is not personal", "Untick to keep the original", "clip:keep", 3.0),
    ("address", "Catch what detection missed", "Select, replace, pick a category", "clip:address", 8.0),
    ("apply", "Apply in one step", "Every mention becomes its alias", "clip:apply", 3.3),
    ("compare", "Check before you share", "Original and aliases, side by side", "clip:compare", 3.6),
    ("copy", "Copy for AI", "With a note to keep every placeholder", "clip:copy", 2.9),
    ("chat", "The AI only sees aliases", "Use any chat you like", "chat", 4.5),
    ("restore", "Bring the reply back", "Real names restored on this computer", "clip:restore", 4.3),
];

pub const IDS: [&str; 13] = [
    "intro", "scan", "detect", "tooltip", "popup", "alias", "keep", "address", "apply", "compare", "copy", "chat",
    "restore",
];

impl Film {
    /// `cut`: scene ids in playing order (any subset, reordered freely); empty for all.
    pub fn new(theme: Theme, cut: &[&str]) -> Self {
        let ids: Vec<&str> = if cut.is_empty() { IDS.to_vec() } else { cut.to_vec() };
        let last = ids.len() - 1;
        let shots = ids
            .iter()
            .enumerate()
            .filter_map(|(i, id)| {
                let &(id, title, sub, content, seconds) = LIBRARY.iter().find(|entry| entry.0 == *id)?;
                let (kind, media) = match content.split_once(':') {
                    Some(("clip", name)) => (Kind::Clip, format!("clip-{name}-{}.mp4", theme.name())),
                    Some(("still", name)) => (Kind::Still, format!("{name}-{}.png", theme.name())),
                    _ if content == "chat" => (Kind::Chat, String::new()),
                    _ => (Kind::Intro, String::new()),
                };
                Some(Shot { id, title, sub, kind, seconds, theme, media, blend: i > 0, handoff: i < last })
            })
            .collect();
        Self { shots }
    }
    fn background(&self) -> &'static str {
        self.shots[0].theme.palette().bg
    }
}

impl Video for Film {
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
        Scenes::from(self.shots.iter().map(|shot| shot as &dyn Scene).collect::<Vec<_>>())
    }
    fn render_frame<'a>(&'a self, frame: Frame, ctx: &FFramesContext<'a, '_>) -> Svgr<'a> {
        fframes::svgr!(
            <svg xmlns="http://www.w3.org/2000/svg" width={WIDTH} height={HEIGHT}>
                <rect width={WIDTH} height={HEIGHT} fill={self.background()} />
                {ctx.render_scenes(&frame)}
            </svg>
        )
    }
}
