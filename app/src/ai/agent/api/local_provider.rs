use std::fs::OpenOptions;
use std::sync::Arc;

use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

/// Debug logging macro that writes to a file, bypassing macOS app daemonization.
macro_rules! debug_log {
    ($($arg:tt)*) => {
        if let Ok(mut f) = OpenOptions::new()
            .create(true)
            .append(true)
            .open("/tmp/local_provider_debug.log")
        {
            use std::io::Write;
            let _ = writeln!(f, "[{}] {}", chrono::Local::now().format("%H:%M:%S%.3f"), format!($($arg)*));
        }
    };
}
use warp_multi_agent_api as api;

use prost_types::FieldMask;

use crate::server::server_api::AIApiError;

use super::{RequestParams, ResponseStream};

const AGENT_OUTPUT_APPEND_MASK_PATH: &str = "agent_output.text";
const RUN_SHELL_COMMAND_TOOL_NAME: &str = "run_shell_command";

/// Configuration for the local AI agent provider.
#[derive(Debug, Clone, Default)]
pub struct LocalProviderConfig {
    pub base_url: String,
    pub model: String,
    pub api_key: String,
}

/// OpenAI-compatible chat completion request.
#[derive(Debug, Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
    stream: bool,
    tools: Vec<ChatTool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ChatMessage {
    role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reasoning_content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    tool_calls: Vec<ChatToolCall>,
}

impl ChatMessage {
    fn system(content: impl Into<String>) -> Self {
        Self {
            role: "system".to_string(),
            content: Some(content.into()),
            reasoning_content: None,
            tool_call_id: None,
            tool_calls: vec![],
        }
    }

    fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".to_string(),
            content: Some(content.into()),
            reasoning_content: None,
            tool_call_id: None,
            tool_calls: vec![],
        }
    }

    fn assistant_reasoning(reasoning_content: impl Into<String>) -> Self {
        Self {
            role: "assistant".to_string(),
            content: Some(String::new()),
            reasoning_content: Some(reasoning_content.into()),
            tool_call_id: None,
            tool_calls: vec![],
        }
    }

    fn assistant_with_reasoning(
        content: impl Into<String>,
        reasoning_content: Option<String>,
    ) -> Self {
        Self {
            role: "assistant".to_string(),
            content: Some(content.into()),
            reasoning_content,
            tool_call_id: None,
            tool_calls: vec![],
        }
    }

    fn assistant_tool_call(tool_call: ChatToolCall, reasoning_content: Option<String>) -> Self {
        Self {
            role: "assistant".to_string(),
            content: None,
            reasoning_content,
            tool_call_id: None,
            tool_calls: vec![tool_call],
        }
    }

    fn tool(tool_call_id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            role: "tool".to_string(),
            content: Some(content.into()),
            reasoning_content: None,
            tool_call_id: Some(tool_call_id.into()),
            tool_calls: vec![],
        }
    }
}

#[derive(Debug, Serialize)]
struct ChatTool {
    r#type: String,
    function: ChatToolFunction,
}

#[derive(Debug, Serialize)]
struct ChatToolFunction {
    name: String,
    description: String,
    parameters: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ChatToolCall {
    id: String,
    r#type: String,
    function: ChatToolCallFunction,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ChatToolCallFunction {
    name: String,
    arguments: String,
}

/// OpenAI-compatible streaming chat completion response.
#[derive(Debug, Deserialize)]
struct ChatCompletionChunk {
    choices: Vec<ChatChunkChoice>,
}

#[derive(Debug, Deserialize)]
struct ChatChunkChoice {
    delta: ChatDelta,
    #[allow(dead_code)]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ChatDelta {
    content: Option<String>,
    #[serde(alias = "reasoning")]
    reasoning_content: Option<String>,
    tool_calls: Option<Vec<ChatToolCallDelta>>,
}

#[derive(Debug, Deserialize, Clone)]
struct ChatToolCallDelta {
    index: usize,
    id: Option<String>,
    r#type: Option<String>,
    function: Option<ChatToolCallFunctionDelta>,
}

#[derive(Debug, Deserialize, Clone)]
struct ChatToolCallFunctionDelta {
    name: Option<String>,
    arguments: Option<String>,
}

#[derive(Debug, Default, Clone)]
struct PendingToolCall {
    id: String,
    r#type: String,
    name: String,
    arguments: String,
}

impl PendingToolCall {
    fn append_delta(&mut self, delta: ChatToolCallDelta) {
        if let Some(id) = delta.id {
            self.id = id;
        }
        if let Some(r#type) = delta.r#type {
            self.r#type = r#type;
        }
        if let Some(function) = delta.function {
            if let Some(name) = function.name {
                self.name.push_str(&name);
            }
            if let Some(arguments) = function.arguments {
                self.arguments.push_str(&arguments);
            }
        }
    }

    fn into_chat_tool_call(self) -> Option<ChatToolCall> {
        if self.name.is_empty() {
            return None;
        }

        Some(ChatToolCall {
            id: if self.id.is_empty() {
                Uuid::new_v4().to_string()
            } else {
                self.id
            },
            r#type: if self.r#type.is_empty() {
                "function".to_string()
            } else {
                self.r#type
            },
            function: ChatToolCallFunction {
                name: self.name,
                arguments: self.arguments,
            },
        })
    }
}

/// Extract the user query text from RequestParams.
#[cfg(test)]
pub fn extract_user_query(params: &RequestParams) -> String {
    for input in &params.input {
        if let crate::ai::agent::AIAgentInput::UserQuery { query, .. } = input {
            return query.clone();
        }
    }
    String::new()
}

fn chat_tools() -> Vec<ChatTool> {
    vec![ChatTool {
        r#type: "function".to_string(),
        function: ChatToolFunction {
            name: RUN_SHELL_COMMAND_TOOL_NAME.to_string(),
            description: "Run a shell command in the active Warp terminal and return its output. Use this only when the user explicitly asks to run a command or when terminal state is required to answer the request. Do not use it for greetings or general conversation.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "command": {
                        "type": "string",
                        "description": "The shell command to run."
                    },
                    "is_read_only": {
                        "type": "boolean",
                        "description": "Whether the command only reads state and does not modify local files, services, or external systems."
                    },
                    "is_risky": {
                        "type": "boolean",
                        "description": "Whether the command may be risky and should require user approval."
                    },
                    "uses_pager": {
                        "type": "boolean",
                        "description": "Whether the command may open an interactive pager."
                    }
                },
                "required": ["command"]
            }),
        },
    }]
}

fn build_chat_messages(params: &RequestParams) -> Vec<ChatMessage> {
    let mut messages = vec![ChatMessage::system(
        "You are Warp Agent running inside a local Warp terminal. \
For greetings, simple chat, and questions that do not require terminal state, answer directly without calling tools. \
Do not run commands just to prove you are working or to demonstrate the terminal. \
Use the run_shell_command tool only when you need to inspect the environment, run terminal commands, or verify work for the user's request. \
When the user asks you to run a specific command, call run_shell_command with that exact command first. \
Do not replace a direct run request with environment checks, summaries, tutorials, or alternative commands unless the command fails or is ambiguous. \
For example, if the user says \"run claude\" or \"帮我运行claude\", run `claude`. \
If the user asks in Chinese, answer in Chinese. \
Prefer read-only commands unless the user asks you to make changes.",
    )];

    for task in &params.tasks {
        let mut pending_reasoning: Option<String> = None;
        for message in &task.messages {
            append_api_message_as_chat(message, &mut messages, &mut pending_reasoning);
        }
        if let Some(reasoning) = pending_reasoning.take() {
            messages.push(ChatMessage::assistant_reasoning(reasoning));
        }
    }

    for input in &params.input {
        append_input_as_chat(input, &mut messages);
    }

    messages
}

fn append_api_message_as_chat(
    message: &api::Message,
    messages: &mut Vec<ChatMessage>,
    pending_reasoning: &mut Option<String>,
) {
    let Some(message) = &message.message else {
        return;
    };

    match message {
        api::message::Message::UserQuery(user_query) => {
            if !user_query.query.is_empty() {
                messages.push(ChatMessage::user(user_query.query.clone()));
            }
        }
        api::message::Message::AgentOutput(agent_output) => {
            if !agent_output.text.is_empty() {
                messages.push(ChatMessage::assistant_with_reasoning(
                    agent_output.text.clone(),
                    pending_reasoning.take(),
                ));
            }
        }
        api::message::Message::AgentReasoning(reasoning) => {
            if !reasoning.reasoning.is_empty() {
                match pending_reasoning {
                    Some(existing) => existing.push_str(&reasoning.reasoning),
                    None => *pending_reasoning = Some(reasoning.reasoning.clone()),
                }
            }
        }
        api::message::Message::ToolCall(tool_call) => {
            if let Some(chat_tool_call) = chat_tool_call_from_api(tool_call) {
                messages.push(ChatMessage::assistant_tool_call(
                    chat_tool_call,
                    pending_reasoning.take(),
                ));
            }
        }
        api::message::Message::ToolCallResult(tool_call_result) => {
            if let Some(content) = tool_result_content_from_api(tool_call_result) {
                messages.push(ChatMessage::tool(
                    tool_call_result.tool_call_id.clone(),
                    content,
                ));
            }
        }
        _ => {}
    }
}

fn append_input_as_chat(input: &crate::ai::agent::AIAgentInput, messages: &mut Vec<ChatMessage>) {
    match input {
        crate::ai::agent::AIAgentInput::UserQuery { query, .. } if !query.is_empty() => {
            messages.push(ChatMessage::user(query.clone()));
        }
        crate::ai::agent::AIAgentInput::ActionResult { result, .. } => {
            messages.push(ChatMessage::tool(
                result.id.to_string(),
                result.result.to_string(),
            ));
        }
        _ => {}
    }
}

fn chat_tool_call_from_api(tool_call: &api::message::ToolCall) -> Option<ChatToolCall> {
    let Some(api::message::tool_call::Tool::RunShellCommand(command)) = &tool_call.tool else {
        return None;
    };

    Some(ChatToolCall {
        id: tool_call.tool_call_id.clone(),
        r#type: "function".to_string(),
        function: ChatToolCallFunction {
            name: RUN_SHELL_COMMAND_TOOL_NAME.to_string(),
            arguments: json!({
                "command": command.command,
                "is_read_only": command.is_read_only,
                "is_risky": command.is_risky,
                "uses_pager": command.uses_pager,
            })
            .to_string(),
        },
    })
}

fn tool_result_content_from_api(tool_call_result: &api::message::ToolCallResult) -> Option<String> {
    let Some(api::message::tool_call_result::Result::RunShellCommand(result)) =
        &tool_call_result.result
    else {
        return None;
    };

    Some(format_run_shell_command_result(result))
}

fn format_run_shell_command_result(result: &api::RunShellCommandResult) -> String {
    match &result.result {
        Some(api::run_shell_command_result::Result::CommandFinished(finished)) => {
            format!(
                "Command `{}` finished with exit code {}.\n\n{}",
                result.command, finished.exit_code, finished.output
            )
        }
        Some(api::run_shell_command_result::Result::LongRunningCommandSnapshot(snapshot)) => {
            format!(
                "Command `{}` is still running. Current output:\n\n{}",
                result.command, snapshot.output
            )
        }
        Some(api::run_shell_command_result::Result::PermissionDenied(_)) => {
            format!("Command `{}` was denied permission to run.", result.command)
        }
        None => format!("Command `{}` returned no result.", result.command),
    }
}

fn tool_call_message_from_chat(
    tool_call: ChatToolCall,
    task_id: &str,
    request_id: &str,
) -> Option<api::Message> {
    if tool_call.function.name != RUN_SHELL_COMMAND_TOOL_NAME {
        debug_log!(
            "Ignoring unsupported tool call: {}",
            tool_call.function.name
        );
        return None;
    }

    let args = serde_json::from_str::<Value>(&tool_call.function.arguments).unwrap_or_default();
    let command = args
        .get("command")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .trim()
        .to_string();
    if command.is_empty() {
        debug_log!("Ignoring run_shell_command tool call with empty command");
        return None;
    }

    let is_read_only = args
        .get("is_read_only")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let is_risky = args
        .get("is_risky")
        .and_then(Value::as_bool)
        .unwrap_or(!is_read_only);
    let uses_pager = args
        .get("uses_pager")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let risk_category = if is_read_only {
        api::RiskCategory::ReadOnly
    } else if is_risky {
        api::RiskCategory::Risky
    } else {
        api::RiskCategory::NontrivialLocalChange
    } as i32;

    Some(api::Message {
        id: Uuid::new_v4().to_string(),
        task_id: task_id.to_string(),
        request_id: request_id.to_string(),
        timestamp: Some(prost_types::Timestamp {
            seconds: chrono::Utc::now().timestamp(),
            nanos: chrono::Utc::now().timestamp_subsec_nanos() as i32,
        }),
        server_message_data: String::new(),
        citations: vec![],
        message: Some(api::message::Message::ToolCall(api::message::ToolCall {
            tool_call_id: tool_call.id,
            tool: Some(api::message::tool_call::Tool::RunShellCommand(
                api::message::tool_call::RunShellCommand {
                    command,
                    is_read_only,
                    uses_pager,
                    citations: vec![],
                    is_risky,
                    wait_until_complete_value: None,
                    risk_category,
                },
            )),
        })),
    })
}

/// Generate a response using the local AI agent provider.
///
/// This calls an OpenAI-compatible /chat/completions endpoint and converts
/// the response into Warp's ResponseEvent stream format.
pub async fn generate_local_agent_output(
    config: LocalProviderConfig,
    params: RequestParams,
    cancellation_rx: futures::channel::oneshot::Receiver<()>,
) -> Result<ResponseStream, Arc<AIApiError>> {
    let chat_messages = build_chat_messages(&params);
    if chat_messages.len() <= 1 {
        let (tx, rx) = async_channel::unbounded();
        let _ = tx
            .send(Err(Arc::new(AIApiError::Other(anyhow::anyhow!(
                "No local provider chat messages found in request"
            )))))
            .await;
        return Ok(Box::pin(rx));
    }

    let conversation_id = params
        .conversation_token
        .as_ref()
        .map(|t| t.as_str().to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    let request_id = Uuid::new_v4().to_string();
    let run_id = Uuid::new_v4().to_string();
    // Use the conversation's root task ID so AddMessagesToTask matches the existing task
    let root_task_id = params.root_task_id.clone().unwrap_or_else(|| {
        debug_log!("WARNING: no root_task_id in params, generating random one");
        Uuid::new_v4().to_string()
    });

    let (tx, rx) = async_channel::unbounded();

    // Spawn the HTTP request in a background task
    let base_url = config.base_url.trim_end_matches('/').to_string();
    let model = config.model.clone();
    let api_key = config.api_key.clone();

    tokio::spawn(async move {
        // Send StreamInit
        let init_event = api::ResponseEvent {
            r#type: Some(api::response_event::Type::Init(
                api::response_event::StreamInit {
                    conversation_id: conversation_id.clone(),
                    request_id: request_id.clone(),
                    run_id: run_id.clone(),
                },
            )),
        };
        if tx.send(Ok(init_event)).await.is_err() {
            return;
        }

        // Build the chat completion request
        let url = format!("{}/chat/completions", base_url);
        debug_log!("Sending request to: {}", url);
        let request_body = ChatCompletionRequest {
            model: model.clone(),
            messages: chat_messages,
            stream: true,
            tools: chat_tools(),
        };

        let client = reqwest::Client::new();
        let response = match client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                let _ = tx
                    .send(Err(Arc::new(AIApiError::Other(anyhow::anyhow!(
                        "Local provider request failed: {}",
                        e
                    )))))
                    .await;
                return;
            }
        };

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            debug_log!("Request failed: {} - {}", status, body);
            let _ = tx
                .send(Err(Arc::new(AIApiError::Other(anyhow::anyhow!(
                    "Local provider returned {}: {}",
                    status,
                    body
                )))))
                .await;
            return;
        }

        debug_log!("Request successful, processing SSE stream");
        // Process SSE stream
        let mut buffer = String::new();
        let mut stream = response.bytes_stream();
        let mut _full_content = String::new();
        let mut full_reasoning = String::new();
        let mut pending_tool_calls: Vec<PendingToolCall> = Vec::new();

        // Use the conversation's root task ID so AddMessagesToTask matches the existing task
        let task_id = root_task_id.clone();

        // Send CreateTask to upgrade the optimistic root task to a server-backed task.
        // Without this, AddMessagesToTask fails with TaskNotInitialized because the
        // root task starts as Optimistic, not Server.
        let create_task_event = api::ResponseEvent {
            r#type: Some(api::response_event::Type::ClientActions(
                api::response_event::ClientActions {
                    actions: vec![api::ClientAction {
                        action: Some(api::client_action::Action::CreateTask(
                            api::client_action::CreateTask {
                                task: Some(api::Task {
                                    id: task_id.clone(),
                                    description: String::new(),
                                    dependencies: None,
                                    messages: vec![],
                                    summary: String::new(),
                                    server_data: String::new(),
                                }),
                            },
                        )),
                    }],
                },
            )),
        };
        if tx.send(Ok(create_task_event)).await.is_err() {
            return;
        }

        let message_id = Uuid::new_v4().to_string();

        // Send the initial assistant message with empty content
        let assistant_msg = api::Message {
            id: message_id.clone(),
            task_id: task_id.clone(),
            request_id: request_id.clone(),
            timestamp: Some(prost_types::Timestamp {
                seconds: chrono::Utc::now().timestamp(),
                nanos: chrono::Utc::now().timestamp_subsec_nanos() as i32,
            }),
            server_message_data: String::new(),
            citations: vec![],
            message: Some(api::message::Message::AgentOutput(
                api::message::AgentOutput {
                    text: String::new(),
                },
            )),
        };

        let add_messages_event = api::ResponseEvent {
            r#type: Some(api::response_event::Type::ClientActions(
                api::response_event::ClientActions {
                    actions: vec![api::ClientAction {
                        action: Some(api::client_action::Action::AddMessagesToTask(
                            api::client_action::AddMessagesToTask {
                                task_id: task_id.clone(),
                                messages: vec![assistant_msg],
                            },
                        )),
                    }],
                },
            )),
        };
        if tx.send(Ok(add_messages_event)).await.is_err() {
            return;
        }

        // Process streaming response
        debug_log!("Starting SSE stream processing");
        'stream_loop: while let Some(chunk) = stream.next().await {
            let chunk = match chunk {
                Ok(c) => c,
                Err(e) => {
                    debug_log!("Stream error: {}", e);
                    let _ = tx
                        .send(Err(Arc::new(AIApiError::Other(anyhow::anyhow!(
                            "Local provider stream error: {}",
                            e
                        )))))
                        .await;
                    return;
                }
            };

            let chunk_str = String::from_utf8_lossy(&chunk);
            debug_log!(
                "Received chunk: {} bytes, preview: {:?}",
                chunk.len(),
                &chunk_str[..chunk_str.len().min(100)]
            );
            buffer.push_str(&chunk_str);

            // Process complete SSE lines
            while let Some(line_end) = buffer.find('\n') {
                let line = buffer[..line_end].trim().to_string();
                buffer = buffer[line_end + 1..].to_string();

                if line.is_empty() || line.starts_with(':') {
                    continue;
                }

                if let Some(data) = line.strip_prefix("data: ") {
                    let data = data.trim();
                    if data == "[DONE]" {
                        debug_log!("Received [DONE]");
                        break 'stream_loop;
                    }

                    debug_log!("Parsing SSE data: {:?}", &data[..data.len().min(100)]);
                    if let Ok(chunk) = serde_json::from_str::<ChatCompletionChunk>(data) {
                        if let Some(choice) = chunk.choices.first() {
                            if let Some(reasoning_content) = &choice.delta.reasoning_content {
                                if !reasoning_content.is_empty() {
                                    full_reasoning.push_str(reasoning_content);
                                }
                            }

                            if let Some(content) = &choice.delta.content {
                                if content.is_empty() {
                                    continue;
                                }
                                debug_log!(
                                    "Got content chunk: {:?}",
                                    &content[..content.len().min(50)]
                                );
                                _full_content.push_str(content);

                                // Send AppendToMessageContent
                                let append_event = api::ResponseEvent {
                                    r#type: Some(api::response_event::Type::ClientActions(
                                        api::response_event::ClientActions {
                                            actions: vec![api::ClientAction {
                                                action: Some(
                                                    api::client_action::Action::AppendToMessageContent(
                                                        api::client_action::AppendToMessageContent {
                                                            task_id: task_id.clone(),
                                                            message: Some(api::Message {
                                                                id: message_id.clone(),
                                                                task_id: task_id.clone(),
                                                                request_id: request_id.clone(),
                                                                timestamp: None,
                                                                server_message_data: String::new(),
                                                                citations: vec![],
                                                                message: Some(
                                                                    api::message::Message::AgentOutput(
                                                                        api::message::AgentOutput {
                                                                            text: content.clone(),
                                                                        },
                                                                    ),
                                                                ),
                                                            }),
                                                            mask: Some(FieldMask {
                                                                paths: vec![AGENT_OUTPUT_APPEND_MASK_PATH.to_string()],
                                                            }),
                                                        },
                                                    ),
                                                ),
                                            }],
                                        },
                                    )),
                                };
                                if tx.send(Ok(append_event)).await.is_err() {
                                    return;
                                }
                            }

                            if let Some(tool_call_deltas) = &choice.delta.tool_calls {
                                for tool_call_delta in tool_call_deltas {
                                    let index = tool_call_delta.index;
                                    if pending_tool_calls.len() <= index {
                                        pending_tool_calls
                                            .resize_with(index + 1, PendingToolCall::default);
                                    }
                                    pending_tool_calls[index].append_delta(tool_call_delta.clone());
                                }
                            }
                        }
                    }
                }
            }
        }

        let mut followup_messages = Vec::new();
        if !full_reasoning.is_empty() {
            followup_messages.push(api::Message {
                id: Uuid::new_v4().to_string(),
                task_id: task_id.clone(),
                request_id: request_id.clone(),
                timestamp: Some(prost_types::Timestamp {
                    seconds: chrono::Utc::now().timestamp(),
                    nanos: chrono::Utc::now().timestamp_subsec_nanos() as i32,
                }),
                server_message_data: String::new(),
                citations: vec![],
                message: Some(api::message::Message::AgentReasoning(
                    api::message::AgentReasoning {
                        reasoning: full_reasoning,
                        finished_duration: None,
                    },
                )),
            });
        }
        followup_messages.extend(
            pending_tool_calls
                .into_iter()
                .filter_map(PendingToolCall::into_chat_tool_call)
                .filter_map(|tool_call| {
                    tool_call_message_from_chat(tool_call, &task_id, &request_id)
                }),
        );
        if !followup_messages.is_empty() {
            debug_log!("Emitting {} follow-up message(s)", followup_messages.len());
            let tool_call_event = api::ResponseEvent {
                r#type: Some(api::response_event::Type::ClientActions(
                    api::response_event::ClientActions {
                        actions: vec![api::ClientAction {
                            action: Some(api::client_action::Action::AddMessagesToTask(
                                api::client_action::AddMessagesToTask {
                                    task_id: task_id.clone(),
                                    messages: followup_messages,
                                },
                            )),
                        }],
                    },
                )),
            };
            if tx.send(Ok(tool_call_event)).await.is_err() {
                return;
            }
        }

        debug_log!(
            "Stream finished, full content length: {}",
            _full_content.len()
        );
        // Send StreamFinished with Done
        let finished_event = api::ResponseEvent {
            r#type: Some(api::response_event::Type::Finished(
                api::response_event::StreamFinished {
                    token_usage: vec![],
                    should_refresh_model_config: false,
                    request_cost: None,
                    conversation_usage_metadata: None,
                    reason: Some(api::response_event::stream_finished::Reason::Done(
                        api::response_event::stream_finished::Done {},
                    )),
                },
            )),
        };
        let _ = tx.send(Ok(finished_event)).await;
    });

    // Wrap with cancellation
    let output_stream = rx.take_until(cancellation_rx);
    Ok(Box::pin(output_stream))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::agent::AIAgentInput;
    use crate::ai::blocklist::SessionContext;
    use crate::ai::llms::LLMId;
    use std::collections::HashMap;

    fn make_test_params(query: &str) -> RequestParams {
        let model = LLMId::from("test-model");
        RequestParams {
            input: vec![AIAgentInput::UserQuery {
                query: query.to_string(),
                context: Arc::new([]),
                static_query_type: None,
                referenced_attachments: HashMap::new(),
                user_query_mode: crate::ai::agent::UserQueryMode::Normal,
                running_command: None,
                intended_agent: None,
            }],
            conversation_token: None,
            forked_from_conversation_token: None,
            ambient_agent_task_id: None,
            tasks: vec![],
            existing_suggestions: None,
            metadata: None,
            session_context: SessionContext::new_for_test(),
            model: model.clone(),
            coding_model: model.clone(),
            cli_agent_model: model.clone(),
            computer_use_model: model,
            is_memory_enabled: false,
            warp_drive_context_enabled: false,
            context_window_limit: None,
            mcp_context: None,
            planning_enabled: true,
            should_redact_secrets: false,
            api_keys: None,
            allow_use_of_warp_credits_with_byok: false,
            autonomy_level: warp_multi_agent_api::AutonomyLevel::Supervised,
            isolation_level: warp_multi_agent_api::IsolationLevel::None,
            web_search_enabled: false,
            computer_use_enabled: false,
            ask_user_question_enabled: false,
            research_agent_enabled: false,
            orchestration_enabled: false,
            supported_tools_override: None,
            parent_agent_id: None,
            agent_name: None,
            local_provider_config: None,
            root_task_id: None,
        }
    }

    #[test]
    fn extract_user_query_from_user_query_input() {
        let params = make_test_params("hello world");
        assert_eq!(extract_user_query(&params), "hello world");
    }

    #[test]
    fn extract_user_query_empty_when_no_user_query() {
        let mut params = make_test_params("");
        params.input = vec![];
        assert_eq!(extract_user_query(&params), "");
    }

    #[test]
    fn response_event_stream_init_has_conversation_id() {
        let event = api::ResponseEvent {
            r#type: Some(api::response_event::Type::Init(
                api::response_event::StreamInit {
                    conversation_id: "test-conv-id".to_string(),
                    request_id: "test-req-id".to_string(),
                    run_id: "test-run-id".to_string(),
                },
            )),
        };
        match &event.r#type {
            Some(api::response_event::Type::Init(init)) => {
                assert_eq!(init.conversation_id, "test-conv-id");
            }
            _ => panic!("Expected Init event"),
        }
    }

    #[test]
    fn response_event_finished_done_has_done_reason() {
        let event = api::ResponseEvent {
            r#type: Some(api::response_event::Type::Finished(
                api::response_event::StreamFinished {
                    token_usage: vec![],
                    should_refresh_model_config: false,
                    request_cost: None,
                    conversation_usage_metadata: None,
                    reason: Some(api::response_event::stream_finished::Reason::Done(
                        api::response_event::stream_finished::Done {},
                    )),
                },
            )),
        };
        match &event.r#type {
            Some(api::response_event::Type::Finished(finished)) => {
                assert!(matches!(
                    finished.reason,
                    Some(api::response_event::stream_finished::Reason::Done(_))
                ));
            }
            _ => panic!("Expected Finished event"),
        }
    }

    #[test]
    fn run_shell_command_tool_call_converts_to_warp_tool_call_message() {
        let tool_call = ChatToolCall {
            id: "call_1".to_string(),
            r#type: "function".to_string(),
            function: ChatToolCallFunction {
                name: RUN_SHELL_COMMAND_TOOL_NAME.to_string(),
                arguments: serde_json::json!({
                    "command": "pwd",
                    "is_read_only": true,
                    "is_risky": false,
                    "uses_pager": false
                })
                .to_string(),
            },
        };

        let message = tool_call_message_from_chat(tool_call, "task_1", "request_1")
            .expect("tool call should convert");

        let Some(api::message::Message::ToolCall(tool_call)) = message.message else {
            panic!("expected ToolCall message");
        };
        let Some(api::message::tool_call::Tool::RunShellCommand(command)) = tool_call.tool else {
            panic!("expected RunShellCommand tool");
        };

        assert_eq!(tool_call.tool_call_id, "call_1");
        assert_eq!(command.command, "pwd");
        assert!(command.is_read_only);
        assert!(!command.is_risky);
        assert_eq!(command.risk_category, api::RiskCategory::ReadOnly as i32);
    }

    #[test]
    fn chat_completion_chunk_deserializes_reasoning_content() {
        let chunk = serde_json::from_value::<ChatCompletionChunk>(serde_json::json!({
            "choices": [{
                "delta": {
                    "reasoning_content": "I should inspect the directory."
                },
                "finish_reason": null
            }]
        }))
        .expect("reasoning_content chunks should deserialize");

        assert_eq!(
            chunk
                .choices
                .first()
                .and_then(|choice| choice.delta.reasoning_content.as_deref()),
            Some("I should inspect the directory.")
        );
    }

    #[test]
    fn build_chat_messages_attaches_reasoning_to_following_tool_call() {
        let mut params = make_test_params("");
        let tool_message = tool_call_message_from_chat(
            ChatToolCall {
                id: "call_1".to_string(),
                r#type: "function".to_string(),
                function: ChatToolCallFunction {
                    name: RUN_SHELL_COMMAND_TOOL_NAME.to_string(),
                    arguments: serde_json::json!({ "command": "pwd" }).to_string(),
                },
            },
            "task_1",
            "request_1",
        )
        .expect("tool call should convert");
        params.input = vec![];
        params.tasks = vec![api::Task {
            id: "task_1".to_string(),
            description: String::new(),
            dependencies: None,
            messages: vec![
                api::Message {
                    id: "reasoning_1".to_string(),
                    task_id: "task_1".to_string(),
                    request_id: "request_1".to_string(),
                    timestamp: None,
                    server_message_data: String::new(),
                    citations: vec![],
                    message: Some(api::message::Message::AgentReasoning(
                        api::message::AgentReasoning {
                            reasoning: "I should inspect the directory.".to_string(),
                            finished_duration: None,
                        },
                    )),
                },
                tool_message,
            ],
            summary: String::new(),
            server_data: String::new(),
        }];

        let messages = build_chat_messages(&params);
        let assistant_tool_call = messages
            .iter()
            .find(|message| !message.tool_calls.is_empty())
            .expect("tool call should be present");

        assert_eq!(
            assistant_tool_call.reasoning_content.as_deref(),
            Some("I should inspect the directory.")
        );
    }

    #[test]
    fn system_prompt_tells_model_not_to_run_commands_for_greetings() {
        let params = make_test_params("你好");
        let messages = build_chat_messages(&params);
        let system_message = messages
            .first()
            .and_then(|message| message.content.as_deref())
            .expect("system message should be present");

        assert!(system_message.contains("For greetings"));
        assert!(system_message.contains("answer directly without calling tools"));
        assert!(system_message.contains("Do not run commands just to prove you are working"));
    }

    #[test]
    fn local_provider_uses_agent_output_append_mask_path() {
        assert_eq!(AGENT_OUTPUT_APPEND_MASK_PATH, "agent_output.text");
    }
}
