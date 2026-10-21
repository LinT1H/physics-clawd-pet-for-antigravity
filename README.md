<div align="center">

# Physics Clawd Pet

**A desktop pet with exaggerated cartoon physics.**<br>
**一只物理效果夸张到离谱的桌宠。**

Grab it, fling it across your screen, and watch it tumble, superhero-land, or stick to the wall like Spider-Man.<br>
把它拎起来甩出去：翻滚落地、超级英雄式砸地，或者像蜘蛛侠一样贴在屏幕边上。

[![Release](https://img.shields.io/github/v/release/Ramirez0291/physics-clawd-pet?include_prereleases&label=release)](https://github.com/Ramirez0291/physics-clawd-pet/releases)
![Platform](https://img.shields.io/badge/platform-Windows%2010%20%7C%2011%20%7C%20macOS-0078D6)
![Tauri](https://img.shields.io/badge/Tauri-2-24C8DB)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)

[English](#english) · [中文](#中文)


![clawd-jump](skins/clawd/gif/clawd-jump.gif)
![clawd-tumble](skins/clawd/gif/clawd-tumble.gif)
![clawd-look](skins/clawd/gif/clawd-look.gif)



</div>

---

## English

### Features

**Throw it around**

- **Pick it up and fling it.** It swings like a pendulum while you hold it. Grab it by a leg and it hangs upside down. Shake it too hard and it gets dizzy.
- **Landings depend on how hard it hits:** a soft landing, a bounce, a roll, a superhero landing, or a faceplant.
- **Sticks to walls and the ceiling** when it hits them fast enough, then crawls along the edge or backflips off.
- **Satisfying impacts:** hit-stop, squash and stretch, motion trails, dust clouds and impact lines.
- **Pixel art or fluff.** Clawd is pixel art: it is rotated on a low-res grid and scaled up with nearest-neighbor, so the pixels stay sharp mid-spin. Dots is drawn smooth at full resolution, with procedurally generated fur.

**It lives on your desktop**

- **Stands on your windows.** It lands on window title bars, hops onto nearby windows, and jumps off the edge.
- **Rides along when you move a window.** Drag the window gently and it rides along. Stop suddenly and inertia flings it off. Yank the window away and it falls.
- **Falls when its window closes,** gets minimized, or is covered by another window.
- **Has a life of its own.** It walks along the taskbar (the Dock on macOS), climbs walls, hangs from the ceiling, and follows your cursor with its eyes. Sometimes it plays on a laptop, trades stocks or eats coins.
- **Roams across all your monitors (Windows).** Your screens form one world: it walks or gets thrown from one to the next. Where one screen is shorter, it jumps down off the ledge or climbs up the taller screen's wall. A screen running a full-screen game or video is off-limits; the others stay in play. On macOS it stays on the main screen for now.
- **Pet it** by moving your cursor back and forth over its head.
- **Stays out of your way.** It walks away from the text box you're typing in, and stays off any screen where a full-screen app (game, video, slides) is running. On macOS, dodging text boxes needs the Accessibility permission: turn it on from the tray menu.
- **Never steals focus or clicks.** Only the pet itself is clickable. Everything else clicks straight through to the windows below.

**It keeps you on schedule**

- **Hourly chime.** On the hour it holds up a bell and rings it once per hour on the clock (only within the hours you choose).
- **Break reminders.** After 45 minutes of continuous keyboard and mouse use it stretches and tells you to take a break. Step away for 5 minutes and the timer resets on its own.
- **Calendar.** Subscribe to any iCal link (Google, Outlook, Feishu, DingTalk…) or pick a local `.ics` file. Before an event starts, Clawd runs to the middle of the screen holding up a calendar page.
- **To-dos.** Give a to-do a due time and Clawd brings you a checklist sign when it's due. Mark it done right from the bubble.
- Reminders appear as a speech bubble above its head. Clawd waits in place until you answer.

**It reacts to Claude Code** (optional, off by default)

- **Turn it on** from the Assistant window and Clawd mirrors what Claude Code is doing: it shows a thinking bubble while Claude thinks, types on a keyboard while it works, sweeps while it compacts context, and cartwheels and throws confetti when a turn finishes.
- **Errors zap it**, and each subagent Claude starts drops a mini Clawd from the sky that types until the subagent is done.
- **Approve from the pet.** When Claude Code asks permission to run a command or edit a file, Clawd runs to that terminal's window and stomps; the bubble above it has Allow / Deny buttons. Answer in the terminal instead and the bubble goes away.
- Each reaction can be turned off separately.

**Seven characters:** Clawd, plus four fluffy Dots: blue with a tilted beret, lime green with eyes on top and a bow tie, yellow with round glasses and a bow tie, and pink with headphones, closed eyes and a pom-pom. Switch from the tray's **Character** menu. Dots uses dense short fur, soft shading and smooth breathing; all four can type, eat tokens and shake tufts of fur loose.
Two pixel characters go with the DeepSeek Harness integration: **DeepSeek-chan**, a chibi whale maid whose arms swing from real shoulder joints, who sits down with her laptop on her lap and changes expression with what she's doing; and **Clawd?**, a Clawd that grew a little blue DeepSeek tail, which spins while the AI is thinking. Clawd now pulls faces too: > < when zapped, ^ ^ when petted, a tear when the stock market tanks.

### Download

Grab it from [**Releases**](https://github.com/Ramirez0291/physics-clawd-pet/releases).

- **Windows:** run the installer. Requires Windows 10 or 11 with WebView2 (already included in Windows 11).
- **macOS:** open the `.dmg` and drag Clawd Pet into Applications. The app isn't notarized by Apple yet, so the first time macOS will refuse to open it: go to System Settings → Privacy & Security and click **Open Anyway**.

### How to use

| Action | How |
| --- | --- |
| To-dos, calendar, reminders | Tray icon → right-click → **Assistant** |
| Throw | Drag Clawd and let go while moving the mouse |
| Pet | Wiggle the cursor back and forth over its head |
| Switch character | Tray icon → right-click → **Character** |
| Change size | Tray icon → right-click → **Size** |
| Lost it? | Tray icon → right-click → **Bring Clawd back** |
| Hide / show | Tray icon → right-click → **Hide Clawd** |
| Tuning panel | Left-click the tray icon |

On macOS the tray icon is Clawd's silhouette in the menu bar.

**Claude Code integration** adds a few entries to the `hooks` section of `~/.claude/settings.json`, all pointing at Clawd. Turning it off removes them, and your own hooks are left alone. The first time Clawd changes that file it saves a backup as `settings.json.clawd-backup`. The Windows uninstaller removes the hooks too. If Clawd is moved to a different folder, open the Assistant window and press **Reconnect**.

The tray menu, the Assistant window, and what Clawd says follow your system language: Chinese, Japanese (where Clawd speaks Osaka dialect — it's an Osaka crab now), or English everywhere else. The tuning panel itself is in Chinese for now.

**Codex integration** is also available: open **Assistant → Claude Code / Codex**, select **Codex**, and enable it. Requires a local Codex CLI or desktop runtime with [lifecycle hooks](https://learn.chatgpt.com/docs/hooks). It reacts to prompts, tools, compaction, subagents, completion, and interruptions. Failed tools trigger the error reaction when their result includes an error flag or a nonzero shell exit status. The pet can also handle permission requests; uncheck bubble approval to receive reminders and answer in Codex instead. Reaction settings are shared with Claude Code, and both integrations can run together.

Codex hooks are added to `$CODEX_HOME/hooks.json` (default `~/.codex/hooks.json`). Existing hooks are preserved; the first edit backs up the file to `hooks.json.clawd-backup`. After enabling, reconnecting, or changing bubble approval, review and trust the updated hooks in Codex (`/hooks` in the CLI), then restart the session if needed. Turning off the integration or uninstalling the pet on Windows removes only its own hooks. Cloud sessions and remote/WSL runtimes cannot connect to this localhost bridge directly.

**DeepSeek Harness integration** works the same way: open **Assistant → Claude Code / Codex / DeepSeek**, select **DeepSeek Harness**, and enable it. DeepSeek Harness has no shell hooks, so the pet installs a small plugin instead: it writes `clawd-pet-<version>.mjs` to `$DSH_HOME/clawd-pet/` (default `~/.dsh/clawd-pet/`) and adds a marked section to the home-level `~/.dsh/cordis.patch.yml`, which applies to every profile. DeepSeek Harness loads it live; no restart is needed. Clawd reacts to prompts, tool calls (a nonzero exit code or a failed tool zaps it), subagents (mini Clawds), compaction, completion, errors and interruptions. Permission requests show up both on Clawd's bubble and in DeepSeek Harness's own approval panel; whichever you answer first wins and the other one goes away. Turning the integration off removes only the pet's section (and the plugin file); the first change to an existing patch file backs it up as `cordis.patch.yml.clawd-backup`. The Windows uninstaller removes it too.

### Tuning panel

Everything about how Clawd feels is adjustable live: gravity, bounciness, landing thresholds, walking speed, size and 60+ other parameters. The panel also has slow motion, pause and single-step, repeatable test throws, and a "replay last throw" button.

Click **Save** to keep your settings. They're stored in `tuning.json` under `%APPDATA%\com.physicsclawdpet.desktop\` (Windows) or `~/Library/Application Support/com.physicsclawdpet.desktop/` (macOS) and loaded on every launch.

### Build from source

Requirements: Node.js 20+ and Rust, plus Visual Studio C++ Build Tools and WebView2 on Windows, or the Xcode Command Line Tools on macOS.

```bash
npm install
npm run tauri dev     # run with the tuning panel open
npm run tauri build   # build the installer → src-tauri/target/release/bundle/ (nsis/ on Windows, dmg/ and macos/ on macOS)
```

Just want to play with the physics? You can skip Rust and preview it in the browser:

```bash
npm run dev           # then open http://localhost:1420 and press D for the tuning panel
npm test              # engine unit tests
```

<details>
<summary>Project layout</summary>

```
src/
  engine/     Physics, state machine and procedural animation. Pure logic, unit-tested.
  render/     Pixel-art rasterizer, smooth/fur renderer and effects
  overlay/    Main loop, mouse input, link to the tuning panel, reminder bubbles
  assistant/  To-dos, iCal parser, reminder scheduler, Assistant window
  debug/      Tuning panel
  platform/   Tauri and browser implementations
  skin/       Skin format and loader
skins/        Built-in characters: clawd/, dots/, dots-green/, dots-yellow/, dots-pink/
src-tauri/    Rust: transparent overlay window, tray, desktop window tracking
  src/platform/   Everything OS-specific: windows/ (Win32) and macos/ (AppKit, CoreGraphics)
tests/        Engine tests
```

</details>

### Custom skins

A skin is a single JSON file: a grid size, a color palette and a list of rectangles. Each rectangle gets one of four roles: `body`, `eye`, `arm` or `leg`. All movement is generated from these roles, so **a new character needs no animation frames at all.** Put it in its own folder under `skins/` and it shows up in the **Character** menu automatically.

A few optional fields:

- `"render": "smooth"` draws the character at full resolution instead of as pixel art. Parts can then be `"shape": "ellipse"` or `"cloud"`, tilted with `"rot"`, and made furry with `"fur": true`.
- `"fur"` on the skin controls `length`, `density` (0.4–2), `curl` (0–1), `light` and `shade`. Fur textures are generated once per size and cached.
- A part's `"fur"` can also be an object with its own settings, as on the blue Dots' dark fabric beret.
- `"outline": 0.22` on a smooth part draws a hollow frame in grid units. Palette colors can use `#rrggbbaa` for transparent lenses and highlights.
- Smooth shapes include `pear` for a rounded belly, `onigiri` for a rounded triangle, `frog` for a seamless body with eye hoods, `bow` for a bow tie, and stroked `arch` / `eyelid` paths for headbands and closed eyes. Closed eye parts follow the body without pupil movement or blinking.
- Eye parts can set `"gaze"` below the default 1 to limit pupil movement within larger eyeballs.
- `"actions"` picks which idle activities it does, from `laptop`, `stocks`, `coin` and `shake`. Leave it out to get the first three.

See [`skins/clawd/skin.json`](skins/clawd/skin.json) (pixel art) and [`skins/dots/skin.json`](skins/dots/skin.json) (smooth and furry) for examples.

Run `npm run dev` and open `http://localhost:1420/preview.html` to compare all four Dots at large and desktop sizes, including petting and typing poses.

### License

The engine code is released under the [MIT License](LICENSE).

Clawd is the mascot of Anthropic's Claude Code. This skin is **fan art**, not affiliated with or endorsed by Anthropic. The character is **not** covered by the MIT license. See [`skins/clawd/README.md`](skins/clawd/README.md).

The Dots skins are likewise **fan art** of OpenAI's Dots, not affiliated with or endorsed by OpenAI, and **not** covered by the MIT license. See [`skins/dots/README.md`](skins/dots/README.md).

---

## 中文

### 能做什么

**甩着玩**

- **拎起来甩出去：** 拎着的时候会像钟摆一样晃，抓腿会倒挂，甩太猛会晕。
- **落地看冲击力：** 普通落地、弹跳、翻滚、超级英雄落地，或者脸着地。
- **贴墙：** 高速撞到屏幕边或天花板会贴住，然后沿着边爬，或者蹬墙后空翻跳下来。
- **打击感拉满：** 顿帧、果冻形变、速度拉伸、残影、烟尘和冲击线。
- **像素风，或者毛茸茸：** Clawd 是像素风，先在低分辨率网格里旋转再最近邻放大，翻滚时像素块也不会糊；Dots 按实际分辨率平滑绘制，毛发是程序生成的。

**住在你的桌面上**

- **站在窗口上：** 会落在窗口顶上，自己跳上附近的窗口，走到边缘再跳下去。
- **跟着窗口走：** 平稳拖动它脚下的窗口会带着它走，急停会把它甩飞，猛地一拽会把窗口从它脚下"抽走"。
- **窗口没了会掉下去：** 脚下的窗口被关掉、最小化或者被挡住时，它会掉下去。
- **自己会玩：** 在任务栏（macOS 上是程序坞）上走来走去、爬墙、倒挂天花板，眼睛跟着鼠标转；有时还会玩电脑、炒股、吃TOKEN。
- **多块屏幕随便跑（Windows）：** 几块显示器拼成一个世界，可以走过去、扔过去；屏幕一高一矮时，矮的那块的边沿会跳下去，高的那块的墙能爬上来。某块屏幕在全屏玩游戏/看视频时，那块屏幕不去，其他屏幕照常。macOS 目前只在主屏上活动。
- **可以摸摸它：** 在它头上来回晃鼠标。
- **不碍事：** 你在哪个输入框打字，它就从那里让开；全屏玩游戏、看视频、放幻灯片时自动躲开那块屏幕。macOS 上躲输入框需要"辅助功能"权限，在托盘菜单里打开。
- **不抢焦点、不挡点击：** 只有宠物本身能点到，其他地方的点击都会穿透到下面的窗口。

**帮你守时间**

- **整点报时：** 到整点举起铃铛摇，几点就摇几下，只在你设定的时段里报。
- **休息提醒：** 连续用电脑（按真实的键鼠操作算）满 45 分钟，伸个大懒腰提醒你起来走走；离开 5 分钟自动重新计时。
- **日历：** 可以订阅任意 iCal 链接（Google、Outlook、飞书、钉钉……），也可以选本地 `.ics` 文件。日程开始前，Clawd 会举着日历页跑到屏幕中间。
- **待办：** 给待办设个截止时间，到点 Clawd 举着清单牌子来提醒，在气泡上就能直接勾掉。
- 提醒会在它头顶冒一个对话气泡，它会站在原地等你回应。

**跟 Claude Code 联动**（可选，默认关闭）

- **在小助手窗口里开启**后，Clawd 会跟着 Claude Code 做事：Claude 思考时头顶冒省略号，干活时敲键盘，压缩上下文时扫地，一轮干完后后空翻撒彩纸。
- **出错会被电一下**；Claude 每派出一个子代理，就从天上掉下一只迷你 Clawd，子代理干活期间它一直敲电脑。
- **在宠物上批准权限。** Claude Code 要跑命令或改文件时，Clawd 会跑到那个终端窗口上跺脚，气泡上有允许 / 拒绝按钮。如果你直接在终端里处理，气泡会自动收起。
- 每种反应都可以单独关掉。

**七个形象：** Clawd，加上四款毛茸茸的 Dots：蓝色斜戴贝雷帽、青柠绿色头顶大眼睛配黑领结、黄色圆眼镜配黑领结、粉色耳机闭眼配头顶毛球。在托盘的 **形象** 菜单里随时切换。Dots 用细密短绒和柔和明暗表现蓬松质感，呼吸和走路起伏也更连贯；四款都会敲代码、吃 TOKEN 和抖毛。
另外有两个配合 DeepSeek Harness 联动的像素形象：**DeepSeek 娘**，Q 版鲸鱼女仆，手臂有真正的肩关节，敲电脑时会坐下来把电脑放在腿上，表情跟着动作变；**Clawd?**，长了一条 DeepSeek 蓝色小尾巴的 Clawd，AI 思考时尾巴甩得飞快。Clawd 现在也有表情了：被电 > <、被摸 ^ ^、炒股亏了掉眼泪。

### 下载

到 [**Releases**](https://github.com/Ramirez0291/physics-clawd-pet/releases) 下载。

- **Windows：** 运行安装程序。系统要求 Windows 10 / 11，需要 WebView2（Windows 11 自带）。
- **macOS：** 打开 `.dmg`，把 Clawd Pet 拖进"应用程序"。目前还没有经过 Apple 公证，第一次打开会被系统拦下：到"系统设置 → 隐私与安全性"里点 **仍要打开**。

### 怎么玩

| 操作 | 方法 |
| --- | --- |
| 待办、日历、提醒 | 托盘图标右键 → **小助手** |
| 甩 | 按住 Clawd 拖动，边移动鼠标边松手 |
| 摸摸 | 在它头上来回晃鼠标 |
| 换形象 | 托盘图标右键 → **形象** |
| 调大小 | 托盘图标右键 → **大小** |
| 找不到它了 | 托盘图标右键 → **把 Clawd 叫回来** |
| 隐藏 / 显示 | 托盘图标右键 → **隐藏 Clawd** |
| 调教面板 | 左键点托盘图标 |

macOS 上托盘图标就是菜单栏里 Clawd 的剪影。

**和 Claude Code 联动**时，会在 `~/.claude/settings.json` 的 `hooks` 里加几条，都指向 Clawd。关闭联动会把它们删掉，不动你自己的 hooks。第一次改之前会备份成 `settings.json.clawd-backup`。在 Windows 上卸载程序时也会一并删除。如果把 Clawd 挪到了别的目录，打开小助手窗口点 **重新开启** 即可。

托盘菜单、小助手窗口和 Clawd 说的话都跟随系统语言：中文系统显示中文，日文系统显示日文（日文版的 Clawd 是只大阪的螃蟹，说关西腔），其他语言显示英文。调教面板目前只有中文。

**和 Codex 联动：** 打开 **小助手 → Claude Code / Codex**，选择 **Codex**，然后开启。需要支持[生命周期 hooks](https://learn.chatgpt.com/docs/hooks) 的本机 Codex CLI 或桌面运行环境。支持思考、工具执行、上下文压缩、子代理、完成和中断；工具结果包含错误标记或非零 shell 退出码时触发被电反应。也可以从气泡处理权限请求；取消勾选气泡批准后，只提醒，仍在 Codex 里选择。反应设置与 Claude Code 共用，两种联动可以同时开启。

Codex 联动写入 `$CODEX_HOME/hooks.json`（默认 `~/.codex/hooks.json`），保留你已有的 hooks，第一次修改前备份为 `hooks.json.clawd-backup`。开启、重新连接或切换气泡批准后，需要在 Codex 里审阅并信任更新后的 hooks（CLI 中使用 `/hooks`），必要时重启会话。关闭联动或在 Windows 上卸载桌宠时，只删除桌宠自己的条目。云端会话以及远程 / WSL 运行环境无法直接连接这个本机桥接服务。

**和 DeepSeek Harness 联动：** 打开 **小助手 → Claude Code / Codex / DeepSeek**，选择 **DeepSeek Harness**，然后开启。DeepSeek Harness 没有 shell hooks，所以桌宠改为装一个小插件：把 `clawd-pet-<版本>.mjs` 放进 `$DSH_HOME/clawd-pet/`（默认 `~/.dsh/clawd-pet/`），并在 home 级的 `~/.dsh/cordis.patch.yml`（对所有 profile 生效）里加一段带标记的配置。DeepSeek Harness 会自动加载，不用重启。支持思考、工具执行（退出码非零或工具报错会被电）、子智能体（迷你 Clawd）、上下文压缩、完成、出错和中断。权限请求会同时出现在 Clawd 的气泡和 DeepSeek Harness 自己的审批面板上，哪边先答算哪边，另一边自动收起。关闭联动只删除桌宠那一段配置和插件文件；第一次修改已有的补丁文件前备份为 `cordis.patch.yml.clawd-backup`。Windows 卸载桌宠时也会一并删除。

### 调教面板

Clawd 的手感全都可以实时调：重力、弹性、落地分级的阈值、走路速度、大小等 60 多个参数。面板里还有慢动作、暂停和单步、可重复的测试投掷，以及"重放上次投掷"。

点 **保存** 就会记住你的设置。设置存在 `tuning.json` 里，Windows 上在 `%APPDATA%\com.physicsclawdpet.desktop\`，macOS 上在 `~/Library/Application Support/com.physicsclawdpet.desktop/`，每次启动自动加载。

### 从源码构建

需要：Node.js 20+ 和 Rust；Windows 上还要 Visual Studio C++ 生成工具和 WebView2，macOS 上还要 Xcode 命令行工具。

```bash
npm install
npm run tauri dev     # 开发模式，会同时打开调教面板
npm run tauri build   # 打包 → src-tauri/target/release/bundle/（Windows 在 nsis/，macOS 在 dmg/ 和 macos/）
```

只想玩物理、不想装 Rust 的话，可以直接在浏览器里预览：

```bash
npm run dev           # 打开 http://localhost:1420，按 D 打开调教面板
npm test              # 引擎单元测试
```

目录结构见上面英文部分的 *Project layout*。

### 自制皮肤

皮肤就是一个 JSON 文件：网格大小、调色板，加上一组矩形部件。每个部件标一个角色：`body`（身体）、`eye`（眼睛）、`arm`（手臂）或 `leg`（腿）。所有动作都按角色程序化生成，所以**做一个新角色不用画任何动画帧**。在 `skins/` 下新建一个目录放进去，它就会自动出现在 **形象** 菜单里。

几个可选字段：

- `"render": "smooth"`：按实际分辨率平滑绘制，不做像素风。这时部件可以用 `"shape": "ellipse"`（椭圆）或 `"cloud"`（云朵），用 `"rot"` 倾斜，加 `"fur": true` 变成毛茸茸的。
- 皮肤的 `"fur"` 可调 `length`（毛长）、`density`（密度，0.4～2）、`curl`（弯曲程度，0～1）、`light` 和 `shade`（亮暗毛色）。毛发贴图按尺寸生成并缓存。
- 部件的 `"fur"` 也可以写成独立的毛发设置对象，例如蓝色 Dots 贝雷帽的深色绒面。
- 平滑部件的 `"outline": 0.22` 可画空心镜框，宽度以格为单位。调色板支持 `#rrggbbaa`，可以画透明镜片和高光。
- 平滑形状还支持 `pear`（圆润梨形）、`bow`（领结），以及配合描边使用的 `arch`（耳机头梁）和 `eyelid`（闭眼曲线）。闭眼部件跟着身体动作，不做瞳孔移动或眨眼压缩。
- 眼睛部件可把 `"gaze"` 调到默认值 1 以下，缩小瞳孔的跟随幅度，让它留在大眼球里面。
- `"actions"`：挑选会做的小动作，可选 `laptop`（敲代码）、`stocks`（炒股）、`coin`（吃TOKEN）、`shake`（抖毛）。不写就是前三个。

可以参考 [`skins/clawd/skin.json`](skins/clawd/skin.json)（像素风）和 [`skins/dots/skin.json`](skins/dots/skin.json)（平滑、毛茸茸）。

运行 `npm run dev`，打开 `http://localhost:1420/preview.html` 可以并排检查四款 Dots 的放大效果、桌面大小、摸摸和敲代码姿势。

### 许可

引擎代码以 [MIT 许可](LICENSE) 开源。

Clawd 是 Anthropic 旗下 Claude Code 的吉祥物。这个皮肤是**同人作品**，与 Anthropic 无关，也不代表官方。Clawd 形象**不在** MIT 许可范围内，详见 [`skins/clawd/README.md`](skins/clawd/README.md)。

Dots 皮肤同样是 OpenAI 的 Dots 的**同人作品**，与 OpenAI 无关，也不代表官方，**不在** MIT 许可范围内，详见 [`skins/dots/README.md`](skins/dots/README.md)。
