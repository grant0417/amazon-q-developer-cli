mod api;
mod parse;
mod prompt;
mod stdio;

use std::io::{
    IsTerminal,
    Read,
    Write,
    stdin,
};
use std::process::ExitCode;
use std::time::Duration;

use color_eyre::owo_colors::OwoColorize;
use crossterm::style::{
    Attribute,
    Color,
};
use crossterm::{
    cursor,
    execute,
    queue,
    style,
    terminal,
};
use eyre::{
    Result,
    bail,
    eyre,
};
use fig_api_client::StreamingClient;
use fig_util::CLI_BINARY_NAME;
use prompt::{
    PROMPT,
    rl,
};
use rustyline::error::ReadlineError;
use spinners::{
    Spinner,
    Spinners,
};
use stdio::StdioOutput;
use winnow::Partial;
use winnow::stream::Offset;

use self::api::send_message;
use self::parse::{
    ParseState,
    interpret_markdown,
};
use crate::util::region_check;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ApiResponse {
    Text(String),
    ConversationId(String),
    MessageId(String),
    Error(Option<String>),
    End,
}

pub async fn chat(mut input: String) -> Result<ExitCode> {
    if !fig_util::system_info::in_cloudshell() && !fig_auth::is_logged_in().await {
        bail!(
            "You are not logged in, please log in with {}",
            format!("{CLI_BINARY_NAME} login",).bold()
        );
    }

    region_check("chat")?;

    let stdin = stdin();
    let is_interactive = stdin.is_terminal();

    if !is_interactive {
        // append to input string any extra info that was provided.
        stdin.lock().read_to_string(&mut input)?;
    }

    let mut output = StdioOutput::new(is_interactive);
    let client = StreamingClient::new().await?;
    let result = try_chat(&mut output, input, is_interactive, &client).await;

    if is_interactive {
        queue!(output, style::SetAttribute(Attribute::Reset), style::ResetColor).ok();
    }
    output.flush().ok();

    result.map(|_| ExitCode::SUCCESS)
}

async fn try_chat<W: Write>(
    output: &mut W,
    mut input: String,
    interactive: bool,
    client: &StreamingClient,
) -> Result<()> {
    let mut rl = if interactive { Some(rl()?) } else { None };
    let mut rx = None;
    let mut conversation_id: Option<String> = None;
    let mut message_id = None;

    loop {
        // Make request with input, otherwise used already provided buffer for input
        if !interactive {
            rx = Some(send_message(client.clone(), input.clone(), conversation_id.clone()).await?);
        } else {
            match input.trim() {
                "exit" | "quit" => {
                    if let Some(conversation_id) = conversation_id {
                        fig_telemetry::send_end_chat(conversation_id.clone()).await;
                    }
                    return Ok(());
                },
                _ => (),
            }

            if !input.is_empty() {
                queue!(output, style::SetForegroundColor(Color::Magenta))?;
                if input.contains("@history") {
                    queue!(output, style::Print("Using shell history\n"))?;
                }

                if input.contains("@git") {
                    queue!(output, style::Print("Using git context\n"))?;
                }

                if input.contains("@env") {
                    queue!(output, style::Print("Using environment\n"))?;
                }

                rx = Some(send_message(client.clone(), input.clone(), conversation_id.clone()).await?);
                queue!(output, style::SetForegroundColor(Color::Reset))?;
                execute!(output, style::Print("\n"))?;
            } else if fig_settings::settings::get_bool_or("chat.greeting.enabled", true) {
                execute!(
                    output,
                    style::Print(color_print::cstr! {"
Hi, I'm Amazon Q. I can answer questions about your shell and CLI tools!
You can include additional context by adding the following to your prompt:

<em>@history</em> to pass your shell history
<em>@git</em> to pass information about your current git repository
<em>@env</em> to pass your shell environment

"
                    })
                )?;
            }
        }

        // Print response as we receive it
        if let Some(rx) = &mut rx {
            let mut spinner = if interactive {
                queue!(output, cursor::Hide)?;
                Some(Spinner::new(Spinners::Dots, "Generating your answer...".to_owned()))
            } else {
                None
            };

            let mut buf = String::new();
            let mut offset = 0;
            let mut ended = false;

            let terminal_width = terminal::window_size().map(|s| s.columns.into()).ok();
            let mut state = ParseState::new(terminal_width);

            loop {
                if let Some(response) = rx.recv().await {
                    match response {
                        ApiResponse::Text(content) => match buf.is_empty() {
                            true => buf.push_str(content.trim_start()),
                            false => buf.push_str(&content),
                        },
                        ApiResponse::ConversationId(id) => {
                            if conversation_id.is_none() {
                                fig_telemetry::send_start_chat(id.clone()).await;

                                tokio::task::spawn(async move {
                                    tokio::signal::ctrl_c().await.unwrap();
                                    if let Some(conversation_id) = &conversation_id {
                                        fig_telemetry::send_end_chat(conversation_id.clone()).await;
                                        fig_telemetry::finish_telemetry().await;
                                        #[allow(clippy::exit)]
                                        std::process::exit(0);
                                    }
                                });
                            }

                            conversation_id = Some(id);
                        },
                        ApiResponse::MessageId(id) => message_id = Some(id),
                        ApiResponse::End => {
                            ended = true;
                        },
                        ApiResponse::Error(error) => {
                            if interactive {
                                drop(spinner.take());
                                queue!(output, cursor::MoveToColumn(0))?;

                                match error {
                                    Some(error) => {
                                        queue!(
                                            output,
                                            style::SetForegroundColor(Color::Red),
                                            style::SetAttribute(Attribute::Bold),
                                            style::Print("error"),
                                            style::SetForegroundColor(Color::Reset),
                                            style::SetAttribute(Attribute::Reset),
                                            style::Print(format!(": {error}\n"))
                                        )?;
                                    },
                                    None => {
                                        queue!(
                                            output,
                                            style::Print(
                                                "Amazon Q is having trouble responding right now. Try again later.",
                                            )
                                        )?;
                                    },
                                };
                            }

                            output.flush()?;
                            ended = true;
                        },
                    }
                }

                // this is a hack since otherwise the parser might report Incomplete with useful data
                // still left in the buffer. I'm not sure how this is intended to be handled.
                if ended {
                    buf.push('\n');
                }

                if !buf.is_empty() && interactive && spinner.is_some() {
                    drop(spinner.take());
                    queue!(
                        output,
                        terminal::Clear(terminal::ClearType::CurrentLine),
                        cursor::MoveToColumn(0),
                        cursor::Show
                    )?;
                }

                loop {
                    let input = Partial::new(&buf[offset..]);
                    // fresh reborrow required on output
                    match interpret_markdown(input, &mut *output, &mut state) {
                        Ok(parsed) => {
                            offset += parsed.offset_from(&input);
                            output.flush()?;
                            state.newline = state.set_newline;
                            state.set_newline = false;
                        },
                        Err(err) => match err.into_inner() {
                            Some(err) => return Err(eyre!(err.to_string())),
                            None => break, // Data was incomplete
                        },
                    }

                    tokio::time::sleep(Duration::from_millis(2)).await;
                }

                if ended {
                    if let (Some(conversation_id), Some(message_id)) = (&conversation_id, &message_id) {
                        fig_telemetry::send_chat_added_message(conversation_id.to_owned(), message_id.to_owned()).await;
                    }

                    if interactive {
                        queue!(output, style::ResetColor, style::SetAttribute(Attribute::Reset))?;

                        for (i, citation) in &state.citations {
                            queue!(
                                output,
                                style::Print("\n"),
                                style::SetForegroundColor(Color::Blue),
                                style::Print(format!("[^{i}]: ")),
                                style::SetForegroundColor(Color::DarkGrey),
                                style::Print(format!("{citation}\n")),
                                style::SetForegroundColor(Color::Reset)
                            )?;
                        }

                        execute!(output, style::Print("\n"))?;
                    }

                    break;
                }
            }
        }

        // rl is Some if the chat is interactive
        if let Some(rl) = rl.as_mut() {
            loop {
                let readline = rl.readline(PROMPT);
                match readline {
                    Ok(line) => {
                        if line.trim().is_empty() {
                            continue;
                        }
                        let _ = rl.add_history_entry(line.as_str());
                        input = line;
                        break;
                    },
                    Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                        return Ok(());
                    },
                    Err(err) => {
                        return Err(err.into());
                    },
                }
            }
        } else {
            break Ok(());
        }
    }
}

#[cfg(test)]
mod test {
    use fig_api_client::model::ChatResponseStream;

    use super::*;

    fn mock_client(s: impl IntoIterator<Item = &'static str>) -> StreamingClient {
        StreamingClient::mock(
            s.into_iter()
                .map(|s| ChatResponseStream::AssistantResponseEvent { content: s.into() })
                .collect(),
        )
    }

    #[tokio::test]
    async fn try_chat_non_interactive() {
        let client = mock_client(["Hello,", " World", "!"]);
        let mut output = Vec::new();
        try_chat(&mut output, "test".into(), false, &client).await.unwrap();

        let mut expected = Vec::new();
        execute!(
            expected,
            style::Print("Hello, World!"),
            style::ResetColor,
            style::SetAttribute(Attribute::Reset),
            style::Print("\n")
        )
        .unwrap();

        assert_eq!(expected, output);
    }

    #[tokio::test]
    async fn try_chat_non_interactive_citation() {
        let client = mock_client(["Citation [[1]](https://aws.com)"]);
        let mut output = Vec::new();
        try_chat(&mut output, "test".into(), false, &client).await.unwrap();

        let mut expected = Vec::new();
        execute!(
            expected,
            style::Print("Citation "),
            style::SetForegroundColor(Color::Blue),
            style::Print("[^1]"),
            style::ResetColor,
            style::ResetColor,
            style::SetAttribute(Attribute::Reset),
            style::Print("\n")
        )
        .unwrap();

        assert_eq!(expected, output);
    }
}
