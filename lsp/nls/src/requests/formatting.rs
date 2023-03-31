use std::process;

use lsp_server::{RequestId, Response, ResponseError};
use lsp_types::{DocumentFormattingParams, Position, Range, TextEdit};

use crate::server::{self, Server};

pub fn handle_format_document(
    params: DocumentFormattingParams,
    id: RequestId,
    server: &mut Server,
) -> Result<(), ResponseError> {
    let document_id = params.text_document.uri.to_file_path().unwrap();
    let file_id = server.cache.id_of(document_id).unwrap();
    let text = server.cache.files().source(file_id);
    let document_length = text.lines().count() as u32;
    let last_line_length = text.lines().rev().next().unwrap().len() as u32;

    let formatting_command = server::FORMATTING_COMMAND;
    let Ok(mut topiary) = process::Command::new(&formatting_command[0])
        .args(&formatting_command[1..])
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .spawn() else {
	    // Also give a notification to tell the user that topiary is not
	    // available
	    return Ok(())
	};

    let mut stdin = topiary.stdin.take().unwrap();
    let mut text_bytes = text.as_bytes();
    let _ = std::io::copy(&mut text_bytes, &mut stdin);

    let output = topiary.wait_with_output().unwrap();

    let new_text = String::from_utf8(output.stdout).unwrap();
    let result = Some(vec![TextEdit {
        range: Range {
            start: Position {
                line: 0,
                character: 0,
            },
            end: Position {
                line: document_length - 1,
                character: last_line_length - 1,
            },
        },
        new_text,
    }]);
    let response = Response::new_ok(id, result);
    server.reply(response);
    Ok(())
}
