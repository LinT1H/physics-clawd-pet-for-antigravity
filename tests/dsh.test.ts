// @ts-nocheck —— 插件是给 DSH 加载的纯 JS，这里还要用 node:net，本项目不带 Node 的类型
// DeepSeek Harness 插件（src-tauri/dsh/clawd-pet.mjs）：用假的 cordis ctx 和假的宠物桥接端口跑一遍
import net from 'node:net';
import { afterEach, describe, expect, it } from 'vitest';
import { apply } from '../src-tauri/dsh/clawd-pet.mjs';

type Listener = (...args: any[]) => any;

/** 假的宠物：记下收到的每一行；PermissionRequest 交给 onPermission 决定怎么回 */
async function fakePet(onPermission: (sock: net.Socket) => void = () => {}) {
  const got: any[] = [];
  const server = net.createServer((sock) => {
    let buf = '';
    sock.on('data', (d) => {
      buf += d.toString();
      if (!buf.includes('\n')) return;
      const msg = JSON.parse(buf.slice(0, buf.indexOf('\n')));
      got.push(msg);
      if (msg.event === 'PermissionRequest') onPermission(sock);
      else sock.end('ok\n');
    });
  });
  await new Promise<void>((r) => server.listen(0, '127.0.0.1', r));
  const port = (server.address() as net.AddressInfo).port;
  return { got, port, close: () => server.close() };
}

function mount(port: number, permission = true) {
  const hooks: Record<string, Listener[]> = {};
  const options: Record<string, unknown> = {};
  const ctx = { on: (name: string, f: Listener, opt?: unknown) => ((hooks[name] ??= []).push(f), (options[name] = opt)) };
  apply(ctx, { port, token: 'tok', permission });
  const session = (id: string, extra: object = {}) => ({ header: { id, cwd: 'E:/work/デモ', ...extra } });
  const ev = (s: object, type: string, data: object = {}) => hooks['session/event'].forEach((f) => f(s, { type, data }));
  return { hooks, options, session, ev };
}

const until = async (cond: () => boolean) => {
  for (let i = 0; i < 100 && !cond(); i++) await new Promise((r) => setTimeout(r, 10));
};

let cleanup: (() => void)[] = [];
afterEach(() => {
  cleanup.forEach((f) => f());
  cleanup = [];
});

describe('DeepSeek Harness plugin', () => {
  it('translates DSH session events into pet hook events', async () => {
    const pet = await fakePet();
    cleanup.push(pet.close);
    const { session, ev } = mount(pet.port);
    const root = session('s1');
    const child = session('c1', { parentSession: 's1', origin: 'subagent', delegationDepth: 1 });
    ev(root, 'turn/start', { turn: 1 });
    ev(root, 'tool/call', { callId: 'a', name: 'pwsh', arguments: '{"command":"npm test","description":"Run"}' });
    ev(root, 'tool/result', { message: { toolCallId: 'a', isError: false, content: [{ type: 'text', text: 'secret\n[exit code: 3]' }] } });
    ev(child, 'turn/start', { turn: 1 });
    ev(child, 'tool/call', { callId: 'b', name: 'pwsh', arguments: '{}' });
    ev(child, 'turn/end', { turn: 1, reason: { kind: 'completed' } });
    ev(root, 'assistant/message', { message: { content: [{ type: 'reasoning', text: 'hmm' }, { type: 'text', text: 'All done.' }] } });
    ev(root, 'turn/end', { turn: 1, reason: { kind: 'completed' } });
    await until(() => pet.got.length >= 6);
    // 连接是并发的，按事件名比较
    const by = (e: string) => pet.got.filter((m) => m.event === e);
    expect(pet.got.map((m) => m.event).sort()).toEqual(
      ['UserPromptSubmit', 'PreToolUse', 'PostToolUseFailure', 'SubagentStart', 'SubagentStop', 'Stop'].sort(),
    );
    expect(pet.got.every((m) => m.provider === 'dsh' && m.token === 'tok' && m.session_id === 'dsh:s1')).toBe(true);
    expect(by('PreToolUse')[0]).toMatchObject({ tool_name: 'pwsh', tool: 'npm test', cwd: 'E:/work/デモ' });
    expect(by('PostToolUseFailure')[0]).toMatchObject({ error: 'Exit code: 3' });
    expect(JSON.stringify(pet.got)).not.toContain('secret');
    expect(by('SubagentStart')[0].agent_id).toBe('dsh:c1');
    expect(by('Stop')[0].last).toBe('All done.');
  });

  it('maps errors and interruptions at turn end', async () => {
    const pet = await fakePet();
    cleanup.push(pet.close);
    const { session, ev } = mount(pet.port);
    const s = session('s2');
    ev(s, 'turn/end', { reason: { kind: 'error', error: { message: 'Sign in required', code: 'X' } } });
    ev(s, 'turn/end', { reason: { kind: 'cancelled' } });
    await until(() => pet.got.length >= 2);
    expect(pet.got.find((m) => m.event === 'StopFailure')?.error).toBe('Sign in required');
    expect(pet.got.some((m) => m.event === 'Interrupt')).toBe(true);
  });

  it('answers approval from the pet and withdraws the DSH prompt', async () => {
    const pet = await fakePet((sock) =>
      sock.end('{"hookSpecificOutput":{"hookEventName":"PermissionRequest","decision":{"behavior":"allow"}}}\n'),
    );
    cleanup.push(pet.close);
    const { hooks, options, session, ev } = mount(pet.port);
    expect(options['approval/request']).toEqual({ prepend: true });
    const s = session('s3');
    ev(s, 'tool/call', { callId: 'c', name: 'pwsh', arguments: '{"command":"rm x"}' });
    const original = new AbortController();
    const req: any = { agent: { session: s }, toolName: 'pwsh', callId: 'c', signal: original.signal };
    let uiSignal: AbortSignal | undefined;
    const ui = () => {
      uiSignal = req.signal;
      return new Promise((r) => req.signal.addEventListener('abort', () => r('cancelled')));
    };
    expect(await hooks['approval/request'][0](req, ui)).toBe('allowed-once');
    expect(uiSignal?.aborted).toBe(true);
    expect(req.signal).toBe(original.signal);
    expect(pet.got.find((m) => m.event === 'PermissionRequest')).toMatchObject({ tool_name: 'pwsh', tool: 'rm x' });
  });

  it('lets DSH answer first and keeps waiting when the pet passes', async () => {
    let petSock: net.Socket | undefined;
    const pet = await fakePet((sock) => {
      petSock = sock;
      sock.write('\n'); // 气泡被关掉 = 交还 DSH
    });
    cleanup.push(pet.close);
    const { hooks, session } = mount(pet.port);
    const s = session('s4');
    let answer: (v: string) => void = () => {};
    const req: any = { agent: { session: s }, toolName: 'pwsh', signal: new AbortController().signal };
    const pending = hooks['approval/request'][0](req, () => new Promise((r) => (answer = r)));
    await until(() => petSock !== undefined);
    answer('rejected');
    expect(await pending).toBe('rejected');
  });

  it('only notifies when bubble approval is off', async () => {
    const pet = await fakePet();
    cleanup.push(pet.close);
    const { hooks, session } = mount(pet.port, false);
    const req: any = { agent: { session: session('s5') }, toolName: 'pwsh', reason: 'why' };
    expect(await hooks['approval/request'][0](req, async () => 'allowed-once')).toBe('allowed-once');
    await until(() => pet.got.length > 0);
    expect(pet.got[0]).toMatchObject({ event: 'Notification', notification_type: 'permission_prompt' });
  });

  it('stays quiet when the pet is not running', async () => {
    const { hooks, session } = mount(1); // 没有人监听的端口
    const req: any = { agent: { session: session('s6') }, toolName: 'pwsh' };
    expect(await hooks['approval/request'][0](req, async () => 'rejected')).toBe('rejected');
  });
});
