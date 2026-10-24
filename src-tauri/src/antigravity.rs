//! Antigravity / agy (Gemini) 生命周期钩子适配器模块。
//! 
//! 【业务背景与原理】
//! 当用户在 Antigravity 中执行任务时，Antigravity 会根据 ~/.gemini/config/hooks.json 的配置，
//! 在特定生命周期节点通过命令行管道执行本项目（clawd-pet.exe），并将事件数据以 JSON 形式通过 stdin 喂进来。
//! 本模块负责：
//! 1. 拦截 Antigravity 专用的命令行标志（--clawd-pet-antigravity-hook）；
//! 2. 从 stdin 读取并解析 Antigravity 的 protojson 报文（清洗字段）；
//! 3. 通过本机 TCP（127.0.0.1:47326）将标准化事件投递给桌宠主程序；
//! 4. 严格按照 Antigravity 契约向 stdout 吐出放行决定（PreToolUse 返回 {"decision":"allow"}，其余返回 {}）；
//! 5. 管理 ~/.gemini/config/hooks.json 策略文件的安装、卸载、自动备份与状态审计。

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::path::{Path, PathBuf};
use std::time::Duration;
use serde_json::{json, Map, Value};
use crate::claude::{HookStatus, PORT};
use crate::platform;

/// Antigravity 命令行启动标志：用于识别本进程是被 Antigravity 钩子触发的，无需启动桌面 UI
pub const MARKER: &str = "--clawd-pet-antigravity-hook";

/// 【路径寻址】定位当前操作系统的 Antigravity 全局钩子配置文件路径
/// 默认位于：%USERPROFILE%\.gemini\config\hooks.json
pub fn settings_path() -> Option<PathBuf> {
    let home = std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME"))?;
    Some(PathBuf::from(home).join(".gemini/config/hooks.json"))
}

/// 获取当前时间的毫秒时间戳（用于给事件打上时间标签，计算耗时）
fn now_ms() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0)
}

/// 字符串长度截断保护：防止超长命令或异常大文本撑爆内存或影响桌宠气泡渲染
fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let mut t: String = s.chars().take(max.saturating_sub(1)).collect();
        t.push('…');
        t
    }
}

/// 【安全审计】检查 hooks.json 中的某条命令是否属于本项目注册的钩子
fn is_ours(h: &Value) -> bool {
    h.get("command")
        .and_then(Value::as_str)
        .is_some_and(|s| s.contains(MARKER))
}

/// 【容错读取】读取并解析 hooks.json，自动去除 Windows PowerShell 常见的 UTF-8 BOM 头
fn read(path: &Path) -> Result<Map<String, Value>, String> {
    if !path.exists() {
        return Ok(Map::new());
    }
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    serde_json::from_str::<Value>(text.trim_start_matches('\u{feff}'))
        .map_err(|e| format!("{}: {e}", path.display()))?
        .as_object()
        .cloned()
        .ok_or_else(|| format!("{}: expected a JSON object", path.display()))
}

/// 【容灾写入】安全原子写入策略文件：
/// 1. 首次修改前自动创建 .clawd-backup 备份文件，确保可追溯、可回滚；
/// 2. 采用“先写临时文件 .clawd-tmp，再操作系统级 rename 替换”的原子写入，避免写到一半断电或崩溃导致原配置损坏。
fn write(path: &Path, settings: Map<String, Value>) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let backup = path.with_extension("json.clawd-backup");
    if path.exists() && !backup.exists() {
        std::fs::copy(path, backup).map_err(|e| e.to_string())?;
    }
    let text = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())? + "\n";
    let tmp = path.with_extension("json.clawd-tmp");
    std::fs::write(&tmp, text).map_err(|e| e.to_string())?;
    std::fs::rename(tmp, path).map_err(|e| e.to_string())
}

/// 生成注入到 Antigravity hooks.json 中的单条调用命令
fn hook_cmd(exe: &Path, token: &str, event: &str) -> String {
    format!("\"{}\" {} {} {} {}", exe.display(), MARKER, PORT, token, event)
}

/// 【策略下发】向 Antigravity hooks.json 注入桌宠监听配置
/// 保留用户已有的其他钩子（如 oil-codex-title），仅新增或覆盖 "clawd-pet" 条目
fn install_into(path: &Path, exe: &Path, token: &str) -> Result<(), String> {
    let mut settings = read(path)?;
    
    // PreToolUse 钩子（工具调用前）：桌宠开始敲笔记本电脑码字，或召唤子智能体
    let pre_tool = json!([
        {
            "matcher": "*",
            "hooks": [
                {
                    "type": "command",
                    "command": hook_cmd(exe, token, "PreToolUse"),
                    "timeout": 5
                }
            ]
        }
    ]);

    // PostToolUse 钩子（工具调用后）：成功则继续，报错则让桌宠触电
    let post_tool = json!([
        {
            "matcher": "*",
            "hooks": [
                {
                    "type": "command",
                    "command": hook_cmd(exe, token, "PostToolUse"),
                    "timeout": 5
                }
            ]
        }
    ]);

    // PreInvocation 钩子（思考开始）：用户发消息后，桌宠头上冒出“…”气泡进入思考
    let pre_invoc = json!([
        {
            "type": "command",
            "command": hook_cmd(exe, token, "PreInvocation"),
            "timeout": 5
        }
    ]);

    // Stop 钩子（轮次结束）：本轮任务完成，桌宠侧手翻落地、撒彩带庆祝
    let stop = json!([
        {
            "type": "command",
            "command": hook_cmd(exe, token, "Stop"),
            "timeout": 5
        }
    ]);

    // 组合为 Antigravity 认可的标准策略结构
    let pet_entry = json!({
        "enabled": true,
        "PreInvocation": pre_invoc,
        "PreToolUse": pre_tool,
        "PostToolUse": post_tool,
        "Stop": stop
    });

    settings.insert("clawd-pet".into(), pet_entry);
    write(path, settings)
}

/// 对外暴露的开启联动方法：定位路径并安装配置
pub fn install(token: &str, _permission: bool) -> Result<(), String> {
    let path = settings_path().ok_or("Could not find Antigravity config folder")?;
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    install_into(&path, &exe, token)
}

/// 对外暴露的关闭联动方法：从 hooks.json 中安全移除 "clawd-pet" 节点，绝不误删用户其他钩子
pub fn uninstall() -> Result<(), String> {
    let Some(path) = settings_path() else { return Ok(()) };
    if !path.exists() {
        return Ok(());
    }
    let mut settings = read(&path)?;
    if settings.remove("clawd-pet").is_some() {
        write(&path, settings)?;
    }
    Ok(())
}

/// 【基线核查】检查 Antigravity 钩子的配置状态（是否已安装、是否被修改、Token 与 exe 路径是否为最新）
fn status_at(path: &Path, exe: &Path, token: &str) -> HookStatus {
    let mut st = HookStatus {
        installed: false,
        permission: false,
        current: true,
        settings_path: path.display().to_string(),
        error: None,
    };
    let settings = match read(path) {
        Ok(s) => s,
        Err(e) => {
            st.error = Some(e);
            return st;
        }
    };
    let Some(entry) = settings.get("clawd-pet").and_then(Value::as_object) else {
        return st;
    };
    if entry.get("enabled").and_then(Value::as_bool) != Some(true) {
        return st;
    }
    st.installed = true;

    // 检查钩子里的命令是否匹配当前的 exe 路径和 Token（如果软件移动了目录，会标记 current=false 自动重新下发）
    let expected_cmd = format!("\"{}\" {} {} {}", exe.display(), MARKER, PORT, token);
    let mut found_expected = false;
    if let Some(list) = entry.get("Stop").and_then(Value::as_array) {
        for h in list {
            if let Some(cmd) = h.get("command").and_then(Value::as_str) {
                if cmd.contains(&expected_cmd) {
                    found_expected = true;
                }
            }
        }
    }
    if !found_expected {
        st.current = false;
    }
    st
}

/// 对外暴露的状态查询接口
pub fn status(token: &str) -> HookStatus {
    match (settings_path(), std::env::current_exe()) {
        (Some(p), Ok(exe)) => status_at(&p, &exe, token),
        _ => HookStatus {
            installed: false,
            permission: false,
            current: false,
            settings_path: String::new(),
            error: Some("Could not find Antigravity hooks path or pet executable".into()),
        },
    }
}

/// 【CLI 中继入口】当被 Antigravity 触发时运行此函数（单次短生命周期进程）：
/// 命令行格式：clawd-pet.exe --clawd-pet-antigravity-hook <端口> <令牌> <事件名>
pub fn hook_main(args: &[String]) -> i32 {
    let i = args.iter().position(|a| a == MARKER).unwrap_or(0);
    let port: u16 = args.get(i + 1).and_then(|p| p.parse().ok()).unwrap_or(PORT);
    let token = args.get(i + 2).cloned().unwrap_or_default();
    let event_type = args.get(i + 3).map(|s| s.as_str()).unwrap_or("");

    // 【重要契约输出闭包】
    // Antigravity 的 PreToolUse 必须返回 {"decision":"allow"} 才能放行工具执行，其余事件返回 {}
    let respond_ok = || {
        if event_type == "PreToolUse" {
            println!("{{\"decision\": \"allow\"}}");
        } else {
            println!("{{}}");
        }
    };

    // 从标准输入（stdin）读取 Antigravity 传递进来的上下文报文（最大 16MB 保护）
    let mut input = String::new();
    let _ = std::io::stdin().take(16 * 1024 * 1024).read_to_string(&mut input);
    let Ok(v) = serde_json::from_str::<Value>(input.trim_start_matches('\u{feff}')) else {
        respond_ok();
        return 0;
    };

    // 清洗报文并附加上 Token 与当前调用链的祖先进程 PID（用于桌宠定位前台窗口）
    let mut msg = summarize(&v, event_type);
    msg.insert("token".into(), Value::String(token));
    msg.insert("ancestors".into(), json!(platform::ancestor_pids()));

    // 建立本机 TCP Socket 连接，向运行中的桌宠主进程（47326端口）投递事件
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    if let Ok(mut stream) = TcpStream::connect_timeout(&addr, Duration::from_millis(400)) {
        let _ = stream.set_write_timeout(Some(Duration::from_secs(3)));
        let mut line = Value::Object(msg).to_string();
        line.push('\n');
        let _ = stream.write_all(line.as_bytes());
        let _ = stream.set_read_timeout(Some(Duration::from_secs(3)));
        let mut reply = String::new();
        let _ = BufReader::new(&stream).take(256 * 1024).read_line(&mut reply);
    }

    // 响应 Antigravity 规范，正常退出
    respond_ok();
    0
}

/// 【协议数据清洗】将 Antigravity 的 protojson 驼峰格式，转换为桌宠引擎通用的蛇形事件格式
pub fn summarize(v: &Value, event_arg: &str) -> Map<String, Value> {
    let mut m = Map::new();
    let mut put = |k: &str, val: Value| {
        if !val.is_null() {
            m.insert(k.into(), val);
        }
    };

    // 提取会话 ID 与项目路径
    let cid = v.get("conversationId").and_then(Value::as_str).unwrap_or("antigravity-session");
    let cwd = v.get("workspacePaths")
        .and_then(Value::as_array)
        .and_then(|a| a.first())
        .and_then(Value::as_str)
        .unwrap_or("");

    put("provider", json!("antigravity"));
    put("session_id", json!(cid));
    put("cwd", json!(cwd));
    put("t", json!(now_ms()));

    let tool_call = v.get("toolCall");
    let tool_name = tool_call.and_then(|t| t.get("name")).and_then(Value::as_str);

    // 根据 Antigravity 的触发点转换桌宠行为
    match event_arg {
        "PreInvocation" => {
            // 开始思考 → 对应桌宠冒“…”气泡
            put("event", json!("UserPromptSubmit"));
        }
        "PreToolUse" => {
            if tool_name == Some("invoke_subagent") {
                // 特殊拦截：若调用了子智能体，桌宠天降迷你 mini-pet！
                let role = tool_call
                    .and_then(|t| t.get("args"))
                    .and_then(|a| a.get("Role").or_else(|| a.get("role")))
                    .and_then(Value::as_str)
                    .unwrap_or("subagent");
                put("event", json!("SubagentStart"));
                put("agent_id", json!(role));
            } else {
                // 普通工具调用 → 桌宠搬出笔记本敲键盘
                put("event", json!("PreToolUse"));
                if let Some(name) = tool_name {
                    put("tool_name", json!(name));
                    put("tool", json!(tool_summary(name, tool_call.and_then(|t| t.get("args")))));
                }
            }
        }
        "PostToolUse" => {
            if tool_name == Some("invoke_subagent") {
                // 子智能体结束 → 收回 mini-pet
                let role = tool_call
                    .and_then(|t| t.get("args"))
                    .and_then(|a| a.get("Role").or_else(|| a.get("role")))
                    .and_then(Value::as_str)
                    .unwrap_or("subagent");
                put("event", json!("SubagentStop"));
                put("agent_id", json!(role));
            } else if let Some(err) = v.get("error").and_then(Value::as_str) {
                // 工具报错 → 桌宠被雷击（> < 表情）
                put("event", json!("PostToolUseFailure"));
                put("error", json!(truncate(err, 200)));
            } else {
                // 工具正常执行完毕
                put("event", json!("PostToolUse"));
                if let Some(name) = tool_name {
                    put("tool_name", json!(name));
                }
            }
        }
        "Stop" => {
            // 对话/任务完成 → 桌宠翻跟头撒花庆祝
            put("event", json!("Stop"));
            put("last", json!("Turn finished"));
        }
        _ => {
            put("event", json!(event_arg));
        }
    }

    m
}

/// 【工具摘要提取】从工具参数中提取最核心的一行文字（如命令行、文件路径），用于气泡展示
fn tool_summary(name: &str, args: Option<&Value>) -> String {
    let Some(args) = args else { return String::new() };
    let s = |k: &str| args.get(k).and_then(Value::as_str);
    let text = match name {
        "run_command" => s("CommandLine").or_else(|| s("command")),
        "read_file" | "write_file" | "edit_file" => s("Path").or_else(|| s("path")),
        "search_web" => s("query"),
        "invoke_subagent" => s("Role").or_else(|| s("Prompt")),
        "ask_question" => s("question"),
        _ => None,
    };
    let text = text.map(str::to_string).or_else(|| {
        args.as_object()?.values().find_map(|x| x.as_str().filter(|t| !t.is_empty()).map(str::to_string))
    });
    truncate(text.unwrap_or_default().trim(), 300)
}
