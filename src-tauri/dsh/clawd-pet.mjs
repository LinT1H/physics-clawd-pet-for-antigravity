// Clawd 桌宠 × DeepSeek Harness。
//
// DeepSeek Harness（下称 DSH）没有 Claude Code 那种 shell hooks，但它是 cordis 插件架构：
// 桌宠把这个文件放进 DSH 目录，再在 ~/.dsh/cordis.patch.yml 里插一行加载它（见 src-tauri/src/dsh.rs）。
// 它在 DSH 进程里监听会话事件，翻译成桌宠认识的 hook 事件名，用和 Claude Code hook 一样的
// 本机 TCP 一行 JSON 协议发给宠物。宠物没开就连不上，静默跳过，不影响 DSH。
//
// 权限请求：挂在 approval/request 瀑布上，同时问宠物和 DSH 自己的界面，谁先答算谁的；
// 先答的那边会把另一边撤掉（宠物答了 → 撤掉 DSH 的确认面板；DSH 里答了 → 宠物气泡收起）。
//
// 只依赖 Node 内置模块，不需要 npm 安装。

import net from 'node:net';

export const name = 'clawd-pet';

/** 权限请求最多等多久（和宠物那边的 PERMISSION_WAIT 对齐，稍长一点） */
const PERMISSION_WAIT_MS = 595_000;
/** 一个会话最多记多少个工具调用（批准请求要用它找命令） */
const MAX_CALLS = 64;

const truncate = (s, max) => (s.length <= max ? s : `${s.slice(0, max - 1)}…`);

/** 工具参数（JSON 字符串）里挑出气泡上显示的一行：命令、文件、网址…… */
function toolSummary(args) {
  let input;
  try {
    input = JSON.parse(args ?? '');
  } catch {
    return truncate(String(args ?? '').trim(), 400);
  }
  if (!input || typeof input !== 'object') return '';
  for (const k of ['command', 'cmd', 'path', 'file_path', 'filePath', 'url', 'query', 'pattern', 'description', 'prompt']) {
    if (typeof input[k] === 'string' && input[k].trim()) return truncate(input[k].trim(), 400);
  }
  const first = Object.values(input).find((v) => typeof v === 'string' && v.trim());
  return truncate((first ?? '').trim(), 400);
}

/** 工具结果的文字部分 */
const resultText = (message) =>
  (message?.content ?? []).filter((b) => b?.type === 'text').map((b) => b.text).join('\n');

/** 失败的工具：isError，或者 shell 退出码不是 0（只留一句简短说明，不带工具输出） */
function failure(message) {
  if (message?.isError === true) return 'Tool failed';
  const m = /\[exit code: (-?\d+)\]\s*$/.exec(resultText(message));
  if (m && m[1] !== '0') return `Exit code: ${m[1]}`;
  return undefined;
}

/** 助手消息里的正文（Stop 时给宠物气泡用） */
const assistantText = (message) =>
  (message?.content ?? []).filter((b) => b?.type === 'text').map((b) => b.text).join(' ').trim();

export function apply(ctx, config) {
  const port = config?.port ?? 47326;
  const token = String(config?.token ?? '');
  const permission = config?.permission !== false;
  // 宠物靠进程 id 找"该跑过去敲的窗口"：DSH 宿主进程的父进程就是带窗口的 Electron 主进程
  const ancestors = [process.pid, process.ppid];

  /** 发一条事件；permission 为真时返回宠物回的那一行（等决定），否则发完就关 */
  function send(msg, { wait = false, signal } = {}) {
    return new Promise((resolve) => {
      const sock = net.connect({ host: '127.0.0.1', port });
      let buf = '';
      let done = false;
      const finish = (v) => {
        if (done) return;
        done = true;
        signal?.removeEventListener('abort', onAbort);
        sock.destroy();
        resolve(v);
      };
      const onAbort = () => finish(null);
      signal?.addEventListener('abort', onAbort, { once: true });
      sock.setTimeout(wait ? PERMISSION_WAIT_MS : 3000, () => finish(null));
      sock.on('error', () => finish(null));
      sock.on('connect', () => {
        sock.write(`${JSON.stringify({ ...msg, provider: 'dsh', t: Date.now(), token, ancestors })}\n`);
      });
      sock.on('data', (d) => {
        buf += d.toString('utf8');
        const nl = buf.indexOf('\n');
        if (nl >= 0) finish(buf.slice(0, nl).trim());
      });
      sock.on('close', () => finish(buf.trim() || null));
    });
  }
  const emit = (msg) => void send(msg);

  /** 会话 id → 这个会话的小账本 */
  const sessions = new Map();
  const info = (session) => {
    const h = session.header ?? {};
    let s = sessions.get(h.id);
    if (!s) {
      s = {
        id: `dsh:${h.id}`,
        cwd: h.cwd ?? '',
        // 子智能体：带 parentSession 的会话，算在父会话头上（宠物给它掉一只迷你 Clawd）
        parent: h.parentSession ? `dsh:${h.parentSession}` : undefined,
        agentId: `dsh:${h.id}`,
        calls: new Map(),
        last: '',
        running: false,
      };
      sessions.set(h.id, s);
    }
    return s;
  };

  ctx.on('session/event', (session, event) => {
    const s = info(session);
    const d = event.data ?? {};
    const base = { session_id: s.parent ?? s.id, cwd: s.cwd };
    if (s.parent) {
      // 子智能体只报开始和结束，它自己的工具调用不算到父会话的账上
      if (event.type === 'turn/start' && !s.running) {
        s.running = true;
        emit({ ...base, event: 'SubagentStart', agent_id: s.agentId });
      } else if (event.type === 'turn/end' && s.running) {
        s.running = false;
        emit({ ...base, event: 'SubagentStop', agent_id: s.agentId });
      }
      return;
    }
    switch (event.type) {
      case 'turn/start':
        s.last = '';
        emit({ ...base, event: 'UserPromptSubmit' });
        break;
      case 'tool/call': {
        const call = { tool_name: d.name ?? '', tool: toolSummary(d.arguments) };
        s.calls.set(d.callId, call);
        if (s.calls.size > MAX_CALLS) s.calls.delete(s.calls.keys().next().value);
        emit({ ...base, event: 'PreToolUse', ...call });
        break;
      }
      case 'tool/result': {
        const m = d.message;
        const call = s.calls.get(m?.toolCallId) ?? {};
        const error = failure(m);
        emit({ ...base, event: error ? 'PostToolUseFailure' : 'PostToolUse', ...call, ...(error ? { error } : {}) });
        break;
      }
      case 'assistant/message': {
        const text = assistantText(d.message);
        if (text) s.last = truncate(text, 160);
        break;
      }
      case 'compaction/start':
        emit({ ...base, event: 'PreCompact', trigger: d.trigger ?? d.reason?.kind ?? 'auto' });
        break;
      case 'compaction/end':
        emit({ ...base, event: 'PostCompact' });
        break;
      case 'turn/end': {
        const kind = d.reason?.kind;
        if (kind === 'completed') emit({ ...base, event: 'Stop', ...(s.last ? { last: s.last } : {}) });
        else if (kind === 'error') {
          const e = d.reason.error ?? {};
          emit({ ...base, event: 'StopFailure', error: truncate(String(e.message ?? e.code ?? 'Error'), 200) });
        } else emit({ ...base, event: 'Interrupt' });
        s.calls.clear();
        break;
      }
    }
  });

  ctx.on('agent/disposed', ({ agent }) => {
    const id = agent?.session?.header?.id;
    const s = sessions.get(id);
    if (!s) return;
    sessions.delete(id);
    if (s.parent) {
      if (s.running) emit({ session_id: s.parent, cwd: s.cwd, event: 'SubagentStop', agent_id: s.agentId });
    } else emit({ session_id: s.id, cwd: s.cwd, event: 'SessionEnd' });
  });

  // prepend：DSH 自己的确认面板也是这个瀑布上的一环，而且不会往下传；排在它前面才轮得到我们
  ctx.on('approval/request', (req, next) => {
    const id = req.agent?.session?.header?.id;
    const s = id ? sessions.get(id) : undefined;
    const call = (req.callId && s?.calls.get(req.callId)) || {
      tool_name: req.toolName ?? '',
      tool: truncate(String(req.reason ?? ''), 400),
    };
    const base = { session_id: s?.parent ?? s?.id ?? `dsh:${id ?? '?'}`, cwd: s?.cwd ?? '', ...call };
    if (!permission) {
      // 没开气泡批准：只提醒一声，照常在 DSH 里选
      emit({ ...base, event: 'Notification', notification_type: 'permission_prompt', message: base.tool });
      return next();
    }

    // DSH 界面那边用我们自己的信号：宠物先答了就撤掉它的确认面板
    const original = req.signal;
    const ui = new AbortController();
    const forward = () => ui.abort(original?.reason);
    if (original?.aborted) ui.abort(original.reason);
    else original?.addEventListener('abort', forward, { once: true });
    req.signal = ui.signal;
    const pet = new AbortController();

    return new Promise((resolve) => {
      let settled = false;
      const settle = (outcome) => {
        if (settled) return;
        settled = true;
        original?.removeEventListener('abort', forward);
        req.signal = original;
        resolve(outcome);
      };
      Promise.resolve()
        .then(() => next())
        .then(
          (outcome) => {
            pet.abort();
            settle(outcome);
          },
          () => {
            pet.abort();
            settle('unavailable');
          },
        );
      void send({ ...base, event: 'PermissionRequest' }, { wait: true, signal: pet.signal }).then((reply) => {
        // 宠物回的是 Claude Code 的 PermissionRequest 输出；空行 = 交还 DSH 自己选
        const behavior = /"behavior"\s*:\s*"(allow|deny)"/.exec(reply ?? '')?.[1];
        if (!behavior || settled) return;
        settle(behavior === 'allow' ? 'allowed-once' : 'rejected');
        ui.abort(new Error('answered from the Clawd desktop pet'));
      });
    });
  }, { prepend: true });
}
