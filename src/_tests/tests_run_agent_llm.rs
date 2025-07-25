//! IMPORTANT: THis file run real agents

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>; // For tests.

use super::*;
use crate::_test_support::{assert_contains, load_test_agent, run_test_agent, run_test_agent_with_input};
use crate::types::FileInfo;
use simple_fs::SPath;
use value_ext::JsonValueExt;

#[tokio::test]
async fn test_run_agent_llm_c_simple_ok() -> Result<()> {
	// -- Setup & Fixtures
	let runtime = Runtime::new_test_runtime_sandbox_01().await?;
	let agent = load_test_agent("./agent-llm/agent-simple.aip", &runtime)?;

	// -- Execute
	let res = run_test_agent(&runtime, &agent).await?;

	// -- Check
	assert_contains(res.as_str().ok_or("Should have output result")?, "sky");

	Ok(())
}

#[tokio::test]
async fn test_run_agent_llm_c_on_file_ok() -> Result<()> {
	// -- Setup & Fixtures
	let runtime = Runtime::new_test_runtime_sandbox_01().await?;
	let agent = load_test_agent("./agent-llm/agent-on-file.aip", &runtime)?;

	// -- Execute
	let on_file = SPath::new("./other/hello.txt");
	let file_info = FileInfo::new(runtime.dir_context(), on_file, false);

	let res = run_test_agent_with_input(&runtime, &agent, file_info).await?;

	// -- Check
	// The output return the {data_path: data.file.path, input_name: input.name}
	assert_eq!(res.x_get_str("data_path")?, "./other/hello.txt");
	assert_eq!(res.x_get_str("input_name")?, "hello.txt");
	let ai_content = res.x_get_str("ai_content")?;
	assert!(ai_content.len() > 10, "The AI response should have some content");
	assert_contains(ai_content, "from the other/hello.txt");

	Ok(())
}

#[tokio::test]
async fn test_run_agent_llm_full_chat_ok() -> Result<()> {
	// -- Setup & Fixtures
	let runtime = Runtime::new_test_runtime_sandbox_01().await?;
	let agent = load_test_agent("agent-llm/agent-full-chat.aip", &runtime)?;

	// -- Execute
	let res = run_test_agent(&runtime, &agent).await?;

	// -- Check
	let content = res.as_str().ok_or("Should return a string")?;
	// Because the `agent-full-chat.aip` system instructs to give only 3 bullet points answer.
	// count the number of lines that starts with -. should 3
	let count = content.lines().filter(|line| line.trim_start().starts_with('-')).count();
	assert_eq!(count, 3, "should have three '-' starting lines");

	Ok(())
}
