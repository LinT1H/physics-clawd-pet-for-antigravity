# Physics Clawd Pet v2.5.0

Clawd now works with DeepSeek Harness, and brings two new pixel friends along.

## New

- **DeepSeek Harness integration.** Open **Assistant → Claude Code / Codex / DeepSeek**, pick **DeepSeek Harness** and turn it on. The pet reacts to prompts, tool calls, failed commands, subagents, compaction and finished turns, just like with Claude Code. DeepSeek Harness has no shell hooks, so the pet installs a small plugin into `~/.dsh` and DeepSeek Harness loads it live, no restart needed.
- **Approve from the pet in DeepSeek Harness too.** Permission requests show up on Clawd's bubble and in DeepSeek Harness's own approval panel at the same time. Answer either one and the other goes away.
- **DeepSeek-chan.** A chibi whale-maid pixel character: wavy blue hair fading to light-blue tips, whale-fin ears, a maid headband with blue bows, a whale embroidered on her apron and a big tail on the floor. Her arms swing from real shoulder joints, her tail, ears and ahoge sway, and she sits down with the laptop on her lap to type.
- **Clawd?** Some AI resellers sell DeepSeek as "Claude". This Clawd grew a little blue DeepSeek tail, drawn after DeepSeek Harness's thinking animation. It spins while the AI is thinking.
- **Expressions.** Faces now change with what the pet is doing: > < when zapped or flung, ^ ^ when petted, wide eyes when picked up, smug after a superhero landing or a stock rally, a tear when stocks crash, a yawn when stretching. Clawd gets the eyes; DeepSeek-chan gets eyes and mouth.

## For skin makers

- Pixel skins can now use real pixel art per part, shoulder joints that turn every existing animation into arm rotation, swaying decorations, parts that stay on the floor, closed-eye frames and per-expression eye and mouth frames. `scripts/pixel-skin.mjs` turns layered ASCII art into a skin; see `scripts/skins/deepseek.mjs`.

## Known issues

- With DeepSeek Harness, approval requests only appear when the AI asks to widen its sandbox. Ordinary commands inside the workspace don't ask.
- DeepSeek-chan's typing hands are hidden behind the laptop lid (we see the back of it). That's on purpose.

## Download

Get the installer from the assets below. It needs WebView2, which Windows 11 includes.

---

# Physics Clawd Pet v2.5.0（中文）

Clawd 现在能和 DeepSeek Harness 联动了，还带来了两个新的像素小伙伴。

## 新增

- **和 DeepSeek Harness 联动。** 打开 **小助手 → Claude Code / Codex / DeepSeek**，选 **DeepSeek Harness** 开启。思考、敲代码、命令出错、子智能体、压缩上下文、干完活，宠物都会像跟 Claude Code 一样跟着动。DeepSeek Harness 没有 shell hooks，所以宠物会往 `~/.dsh` 里装一个小插件，DeepSeek Harness 自动加载，不用重启。
- **DeepSeek Harness 的权限请求也能在宠物上批。** 请求会同时出现在 Clawd 的气泡和 DeepSeek Harness 自己的审批面板上，哪边先答算哪边，另一边自动收起。
- **DeepSeek 娘。** Q 版鲸鱼女仆像素小人：蓝色卷发渐变成浅蓝发梢、鲸鱼鳍耳朵、女仆头饰配蓝蝴蝶结、围裙上绣着小鲸鱼、身后拖着大尾巴。手臂有真正的肩关节，尾巴、耳朵、呆毛会自己晃，敲电脑时会坐下来把电脑放在腿上。
- **Clawd?** 有些 AI 中转站拿 DeepSeek 冒充 Claude 卖。于是这只 Clawd 长出了一条 DeepSeek 的蓝色小尾巴，照着 DeepSeek Harness 思考时的动画画的，AI 思考时尾巴甩得飞快。
- **表情。** 表情跟着动作变：被电、被甩 > <，被摸 ^ ^，被拎起来瞪大眼，超级英雄落地、炒股大涨时得意，炒股大跌掉眼泪，伸懒腰打哈欠。Clawd 换眼睛，DeepSeek 娘眼睛和嘴都会变。

## 给做皮肤的人

- 像素皮肤现在可以给每个部件画真正的像素图，给手臂加肩关节（所有现成的动作都会自动换成绕肩旋转），加会晃的装饰、贴地不动的部件、闭眼图，以及每种表情的眼睛和嘴巴。`scripts/pixel-skin.mjs` 能把分层的 ASCII 像素图拼成皮肤，参考 `scripts/skins/deepseek.mjs`。

## 已知问题

- DeepSeek Harness 只有在 AI 要扩大沙盒权限时才会弹审批，工作区内的普通命令不会问。
- DeepSeek 娘敲键盘的手藏在电脑屏幕后面（我们看到的是屏幕背面），这是故意的。

## 下载

在下方 Assets 中获取安装包。需要 WebView2（Windows 11 自带）。
