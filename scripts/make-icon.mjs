// 把皮肤的静止姿态栅格化成 1024×1024 PNG，供 `tauri icon` 生成各尺寸图标。
// 用法: node scripts/make-icon.mjs [skins/clawd/skin.json] [app-icon.png]
import { readFileSync, writeFileSync } from 'node:fs';
import { deflateSync } from 'node:zlib';

const skinPath = process.argv[2] ?? 'skins/clawd/skin.json';
const outPath = process.argv[3] ?? 'app-icon.png';
const skin = JSON.parse(readFileSync(skinPath, 'utf8'));

const SIZE = 1024;
const [gw, gh] = skin.grid;
const unit = Math.floor((SIZE * 0.9) / Math.max(gw, gh));
const ox = Math.floor((SIZE - gw * unit) / 2);
const oy = Math.floor((SIZE - gh * unit) / 2);

const hex = (h) => [1, 3, 5].map((i) => parseInt(h.slice(i, i + 2), 16));
const pixels = Buffer.alloc(SIZE * SIZE * 4);
const fill = (x0, y0, x1, y1, color) => {
  const [r, g, b] = hex(color);
  for (let y = Math.round(y0); y < Math.round(y1); y++) {
    for (let x = Math.round(x0); x < Math.round(x1); x++) {
      if (x < 0 || y < 0 || x >= SIZE || y >= SIZE) continue;
      const i = (y * SIZE + x) * 4;
      pixels[i] = r;
      pixels[i + 1] = g;
      pixels[i + 2] = b;
      pixels[i + 3] = 255;
    }
  }
};
// 部件可以是纯色矩形，也可以是像素图（一个字符 = 1/3 格，字符按 legend 换成颜色，'.' 透明）
for (const part of skin.parts) {
  const [px, py, pw, ph] = part.rect;
  const x0 = ox + px * unit;
  const y0 = oy + py * unit;
  if (!part.pixels) {
    fill(x0, y0, x0 + pw * unit, y0 + ph * unit, skin.palette[part.color]);
    continue;
  }
  const sub = unit / 3;
  part.pixels.forEach((row, j) =>
    [...row].forEach((c, i) => {
      if (c === '.' || c === ' ') return;
      fill(x0 + i * sub, y0 + j * sub, x0 + (i + 1) * sub, y0 + (j + 1) * sub, skin.palette[skin.legend[c]]);
    }),
  );
}

const crcTable = Array.from({ length: 256 }, (_, n) => {
  let c = n;
  for (let k = 0; k < 8; k++) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1;
  return c >>> 0;
});
const crc32 = (buf) => {
  let c = 0xffffffff;
  for (const byte of buf) c = crcTable[(c ^ byte) & 0xff] ^ (c >>> 8);
  return (c ^ 0xffffffff) >>> 0;
};
const chunk = (type, data) => {
  const len = Buffer.alloc(4);
  len.writeUInt32BE(data.length);
  const body = Buffer.concat([Buffer.from(type, 'ascii'), data]);
  const crc = Buffer.alloc(4);
  crc.writeUInt32BE(crc32(body));
  return Buffer.concat([len, body, crc]);
};

const ihdr = Buffer.alloc(13);
ihdr.writeUInt32BE(SIZE, 0);
ihdr.writeUInt32BE(SIZE, 4);
ihdr[8] = 8; // bit depth
ihdr[9] = 6; // RGBA
const raw = Buffer.alloc((SIZE * 4 + 1) * SIZE);
for (let y = 0; y < SIZE; y++) {
  raw[y * (SIZE * 4 + 1)] = 0; // filter: none
  pixels.copy(raw, y * (SIZE * 4 + 1) + 1, y * SIZE * 4, (y + 1) * SIZE * 4);
}
const png = Buffer.concat([
  Buffer.from([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]),
  chunk('IHDR', ihdr),
  chunk('IDAT', deflateSync(raw)),
  chunk('IEND', Buffer.alloc(0)),
]);
writeFileSync(outPath, png);
console.log(`wrote ${outPath}`);
