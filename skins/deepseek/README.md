# DeepSeek 娘（鲸鱼女仆）

DeepSeek 的拟人形象：蓝色长卷发（发梢渐变成浅蓝）、鲸鱼鳍耳朵、女仆头饰和浅蓝蝴蝶结、
藏青女仆裙配绣着小鲸鱼的白围裙、拖在地上的鲸鱼尾巴。按 Gemini 生成的 Q 版像素设定图逐格描下来，再拆成能动的部件。这是同人皮肤，与 DeepSeek 无关，也不代表官方；
相关权利归各自所有者，**不在本项目引擎代码的开源许可范围内**。

## 怎么改

`skin.json` 是生成出来的，别直接改。像素图在 [`scripts/skins/deepseek.mjs`](../../scripts/skins/deepseek.mjs)，
一个字符一个像素、每个图层一个部件。改完运行：

```bash
npm run skin:deepseek                 # 重新生成 skin.json
node scripts/skins/deepseek.mjs a.png # 顺便输出一张放大的预览图
```

## 拟人骨骼

和 Clawd 一样只用 body / eye / arm / leg 四种部件，额外用到了像素皮肤的几个可选字段：

- `pixels` + `legend`：部件是一张像素图，透明的地方露出下面的部件。
- `pivot`：手臂的肩关节。动作里"把手抬高"会换算成绕肩旋转，所以所有动作都不用另写。
- `sway`：尾巴、耳朵、呆毛绕关节自己晃，走路晃得快，被甩飞时往后甩。
- `closed`：眨眼、眯眼时换的闭眼图。
- `faces`：眼睛、嘴巴的表情图。动作会按状态挑表情：被摸、庆祝 happy（^^），被拎起来、飞着 surprised（○○），
  被甩、被电、摔扁 wince（><），炒股赚了、超级英雄落地 smug，亏了 cry（掉眼泪），敲电脑 focus，
  吃金币 eat，伸懒腰 yawn。没画的表情就用平时的样子。
- `anchored`：尾巴拖在地上，坐下、呼吸时不跟着身子动。
- 皮肤的 `"laptop": "lap"`：敲电脑、炒股时坐下来，电脑放在腿上（看到屏幕背面的鲸鱼标志，炒股时标志跟着涨跌变红变绿），两只手伸到键盘上轮流敲。

做新的拟人像素角色，照着 `scripts/skins/deepseek.mjs` 抄一份改图层就行。
