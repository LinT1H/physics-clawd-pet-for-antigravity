//! 桌面几何：窗口矩形、可站立的窗口顶边。与平台无关，查询系统的部分在 platform 里。

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Rect {
    pub fn width(&self) -> i32 {
        self.right - self.left
    }
    pub fn height(&self) -> i32 {
        self.bottom - self.top
    }
    /// 外接矩形
    pub fn union(&self, o: &Rect) -> Rect {
        Rect {
            left: self.left.min(o.left),
            top: self.top.min(o.top),
            right: self.right.max(o.right),
            bottom: self.bottom.max(o.bottom),
        }
    }
}

/// A display in desktop units: physical pixels on Windows, logical points on macOS.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Screen {
    /// 整块屏幕
    pub monitor: Rect,
    /// 去掉任务栏（macOS：菜单栏和程序坞）之后的工作区
    pub work: Rect,
    pub primary: bool,
}

/// 覆盖层的摆法：宠物能去的屏幕，以及覆盖层要铺的范围（这些屏幕工作区的外接矩形）
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Layout {
    pub rect: Rect,
    pub screens: Vec<Screen>,
}

/// 被全屏程序占着的屏幕不让宠物去，覆盖层也不盖上去（免得挡住全屏游戏的独占翻转）。
/// 一块能去的都没有返回 None（隐藏覆盖层）。
pub fn layout(screens: &[Screen], fullscreen: &[Rect]) -> Option<Layout> {
    let screens: Vec<Screen> = screens.iter().filter(|s| !fullscreen.contains(&s.monitor)).copied().collect();
    let rect = screens.iter().map(|s| s.work).reduce(|a, b| a.union(&b))?;
    Some(Layout { rect, screens })
}

/// macOS renders through a viewport on one display. Keep the current viewport in
/// gaps between displays; choose another only when the pet enters its monitor.
/// If a display disappears or becomes fullscreen, fall back to an available one.
#[cfg(any(target_os = "macos", test))]
pub fn viewport(screens: &[Screen], current: Rect, anchor: Option<(f64, f64)>) -> Rect {
    anchor
        .and_then(|(x, y)| {
            screens.iter().find(|s| {
                x >= s.monitor.left as f64
                    && x < s.monitor.right as f64
                    && y >= s.monitor.top as f64
                    && y < s.monitor.bottom as f64
            })
        })
        .or_else(|| screens.iter().find(|s| s.work == current))
        .or_else(|| screens.iter().find(|s| s.primary))
        .or_else(|| screens.first())
        .map_or(current, |s| s.work)
}

/// 覆盖层从 (old, old_scale) 挪到 (new, new_scale) 后，同一个屏幕上的点 CSS 坐标怎么变：
/// 新 = 旧 × k + (dx, dy)
pub fn remap(old: Rect, old_scale: f64, new: Rect, new_scale: f64) -> (f64, f64, f64) {
    (old_scale / new_scale, (old.left - new.left) as f64 / new_scale, (old.top - new.top) as f64 / new_scale)
}

/// 一个顶层窗口。`platform` 表示它的顶边可以站人；不能站的窗口仍然参与遮挡计算。
#[derive(Clone, Copy, Debug)]
pub struct WinInfo {
    pub id: isize,
    pub rect: Rect,
    pub platform: bool,
}

/// 可站立的一段窗口顶边（物理像素）
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Segment {
    pub id: isize,
    pub x0: i32,
    pub x1: i32,
    pub y: i32,
}

/// 计算可站立的顶边：每个窗口的顶边减去被它上面（Z 序更高）的窗口挡住的部分，再裁到工作区内。
pub fn platform_segments(wins: &[WinInfo], area: Rect, min_len: i32) -> Vec<Segment> {
    let mut out = Vec::new();
    for (i, w) in wins.iter().enumerate() {
        if !w.platform {
            continue;
        }
        let y = w.rect.top;
        // 顶边贴着工作区顶部（比如最大化窗口）就没地方站了
        if y <= area.top || y >= area.bottom {
            continue;
        }
        let mut segs = vec![(w.rect.left.max(area.left), w.rect.right.min(area.right))];
        for above in &wins[..i] {
            let r = above.rect;
            if r.top < y && r.bottom >= y {
                segs = segs
                    .into_iter()
                    .flat_map(|(a, b)| {
                        let mut parts = Vec::with_capacity(2);
                        if r.left > a {
                            parts.push((a, b.min(r.left)));
                        }
                        if r.right < b {
                            parts.push((a.max(r.right), b));
                        }
                        parts
                    })
                    .filter(|(a, b)| b > a)
                    .collect();
                if segs.is_empty() {
                    break;
                }
            }
        }
        for (x0, x1) in segs {
            if x1 - x0 >= min_len {
                out.push(Segment { id: w.id, x0, x1, y });
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn win(id: isize, l: i32, t: i32, r: i32, b: i32) -> WinInfo {
        WinInfo { id, rect: Rect { left: l, top: t, right: r, bottom: b }, platform: true }
    }
    const AREA: Rect = Rect { left: 0, top: 0, right: 1920, bottom: 1040 };

    #[test]
    fn top_edge_is_split_by_window_above() {
        // 窗口 2 在窗口 1 上面，盖住了窗口 1 顶边的中间一段
        let wins = [win(2, 400, 100, 700, 600), win(1, 200, 300, 1000, 800)];
        let segs = platform_segments(&wins, AREA, 10);
        assert_eq!(
            segs,
            vec![
                Segment { id: 2, x0: 400, x1: 700, y: 100 },
                Segment { id: 1, x0: 200, x1: 400, y: 300 },
                Segment { id: 1, x0: 700, x1: 1000, y: 300 },
            ]
        );
    }

    #[test]
    fn window_below_the_edge_does_not_occlude() {
        // 上层窗口的顶边比下层窗口顶边还低：挡不住下层窗口的顶边
        let wins = [win(2, 0, 500, 1920, 900), win(1, 200, 300, 1000, 800)];
        let segs = platform_segments(&wins, AREA, 10);
        assert!(segs.contains(&Segment { id: 1, x0: 200, x1: 1000, y: 300 }));
    }

    #[test]
    fn maximized_window_hides_everything_behind_it_and_has_no_room_on_top() {
        let mut max = win(9, 0, 0, 1920, 1040);
        max.platform = true;
        let wins = [max, win(1, 200, 300, 1000, 800)];
        assert!(platform_segments(&wins, AREA, 10).is_empty());
    }

    fn screen(l: i32, t: i32, r: i32, b: i32, taskbar: i32, primary: bool) -> Screen {
        let monitor = Rect { left: l, top: t, right: r, bottom: b };
        Screen { monitor, work: Rect { bottom: b - taskbar, ..monitor }, primary }
    }

    #[test]
    fn overlay_spans_every_screen_not_taken_by_a_fullscreen_app() {
        let main = screen(0, 0, 1920, 1080, 48, true);
        let side = screen(1920, -200, 4480, 1240, 48, false);
        let all = layout(&[main, side], &[]).unwrap();
        assert_eq!(all.rect, Rect { left: 0, top: -200, right: 4480, bottom: 1192 });
        assert_eq!(all.screens.len(), 2);

        let only_main = layout(&[main, side], &[side.monitor]).unwrap();
        assert_eq!(only_main.rect, main.work);
        assert_eq!(only_main.screens, vec![main]);

        assert!(layout(&[main, side], &[main.monitor, side.monitor]).is_none());
    }

    #[test]
    fn viewport_follows_pet_between_stacked_mixed_density_displays() {
        let mac = screen(0, 0, 1728, 1117, 61, true);
        let lg = screen(-868, -1440, 2572, 0, 0, false);
        let screens = [mac, lg];
        assert_eq!(viewport(&screens, mac.work, Some((800.0, -500.0))), lg.work);
        assert_eq!(viewport(&screens, lg.work, Some((800.0, 500.0))), mac.work);
        // Gaps do not cause a flip back to the primary screen.
        assert_eq!(viewport(&screens, lg.work, Some((-500.0, 500.0))), lg.work);
        // Unplugging or fullscreen exclusion recovers the remaining screen.
        assert_eq!(viewport(&[mac], lg.work, Some((800.0, -500.0))), mac.work);
        assert_eq!(viewport(&[lg], mac.work, Some((800.0, 500.0))), lg.work);
        let (k, dx, dy) = remap(mac.work, 1.0, lg.work, 1.0);
        assert_eq!((k, 800.0 * k + dx, -500.0 * k + dy), (1.0, 1668.0, 940.0));
    }

    #[test]
    fn remap_keeps_points_on_the_same_physical_pixel() {
        let old = Rect { left: 0, top: 0, right: 3840, bottom: 1032 };
        let new = Rect { left: 1920, top: 0, right: 3840, bottom: 1032 };
        // 物理像素 2400 在旧覆盖层（缩放 1.5）里是 CSS 1600，在新覆盖层（缩放 1）里是 480
        let (k, dx, dy) = remap(old, 1.5, new, 1.0);
        assert!((1600.0 * k + dx - 480.0).abs() < 1e-9);
        assert_eq!(dy, 0.0);
    }

    #[test]
    fn segments_are_clipped_to_work_area_and_min_length() {
        let wins = [win(1, -300, 200, 50, 500), win(2, 1900, 400, 2400, 700)];
        let segs = platform_segments(&wins, AREA, 30);
        assert_eq!(segs, vec![Segment { id: 1, x0: 0, x1: 50, y: 200 }]);
    }
}
