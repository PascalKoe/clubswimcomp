use std::{collections::HashMap, time::Duration};

use anyhow::Context;
use tokio::io::AsyncWriteExt;
use tracing::instrument;

/// Output format to be generated by the Typst compiler
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TypstOutput {
    Pdf,
    Png,
    Svg,
}

impl TypstOutput {
    fn as_cli_arg(&self) -> &'static str {
        match self {
            TypstOutput::Pdf => "pdf",
            TypstOutput::Png => "png",
            TypstOutput::Svg => "svg",
        }
    }
}

/// Compile a typst template to the given output format.
///
/// # Parameters:
/// - `template` - The typst template
/// - `output_format` - The format to compile the document to
/// - `inputs` - List of key-value pairs accessible in typst using `sys.inputs.<key>`
#[instrument]
pub async fn compile(
    template: &str,
    output_format: TypstOutput,
    inputs: &HashMap<String, String>,
) -> anyhow::Result<Vec<u8>> {
    use std::process::Stdio;
    use tokio::process::Command;

    let args = build_cli_args(output_format, inputs);

    // TODO: Make the typst binary path configurable
    let mut typst_process = Command::new("typst")
        .env_clear()
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .args(args)
        .spawn()
        .context("Could not spawn typst as a command")?;

    // Pipe the template to the STDIN of the process
    typst_process
        .stdin
        .take()
        .context("Could not take STDIN of the spawned typst process")?
        .write_all(template.as_bytes())
        .await
        .context("Failed to pipe template into typst process")?;

    let max_compile_time = Duration::from_secs(10);
    let typst_output =
        match tokio::time::timeout(max_compile_time, typst_process.wait_with_output()).await {
            Ok(Ok(typst_output)) => Ok(typst_output),
            Ok(Err(e)) => Err(e).context("Typst process run into a error"),
            Err(e) => Err(e).context("Compilation did not finish within timeout limit"),
        }?;

    // Typst compilation failed
    if !typst_output.status.success() {
        let typst_error_output = &String::from_utf8_lossy(&typst_output.stderr)[..];
        tracing::warn!(typst_error_output, "Typst compilation failed with error");
        anyhow::bail!("Typst compilation failed with error")
    }

    Ok(typst_output.stdout)
}

#[instrument]
fn build_cli_args(output_format: TypstOutput, inputs: &HashMap<String, String>) -> Vec<String> {
    let mut args = vec![
        "compile".to_string(),
        "--format".to_string(),
        output_format.as_cli_arg().to_string(),
    ];

    // Append the input parameters to the args
    for (key, value) in inputs.iter() {
        args.push("--input".to_string());
        args.push(format!("{key}={value}"));
    }

    // Use STDIN as the input file
    args.push("-".to_string());

    // Use STDOUT as the output file
    args.push("/dev/stdout".to_string());

    args
}