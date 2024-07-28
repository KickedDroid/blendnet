use std::error::Error;
use std::net::IpAddr;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct RenderNode {
    id: String,
    ip_address: IpAddr,
    available_threads: u32,
    status: NodeStatus,
}

#[derive(Debug, Clone)]
pub enum NodeStatus {
    Idle,
    Rendering,
    Offline,
}

#[derive(Debug, Clone)]
pub struct RenderTask {
    pub id: String,
    pub blend_file: PathBuf,
    pub start_frame: u32,
    pub end_frame: u32,
    pub render_engine: RenderEngine,
    pub threads: u32,
}

#[derive(Debug, Clone)]
pub enum RenderEngine {
    Cycles,
    Eevee,
    // Add other engines as needed
}

#[derive(Debug, Clone)]
pub enum TaskStatus {
    Queued,
    InProgress,
    Completed,
    Failed,
}

#[derive(Debug)]
pub(crate) struct ControlNode {
    render_nodes: Vec<RenderNode>,
    task_queue: Vec<RenderTask>,
    completed_tasks: Vec<RenderTask>,
}

#[derive(Debug)]
pub(crate) struct RenderResult {
    task_id: String,
    frame_number: u32,
    output_file: PathBuf,
}

pub fn execute_render(task: &RenderTask) -> Result<(), Box<dyn Error>> {
    let args = construct_blender_task(task);
    
    let output = Command::new("blender")
        .args(&args)
        .output()?;
    
    println!("Render complete. Output: {}", String::from_utf8_lossy(&output.stdout));
    
    if !output.status.success() {
        println!("Error: {}", String::from_utf8_lossy(&output.stderr));
        return Err("Blender render failed".into());
    }
    
    Ok(())
}

fn construct_blender_task(task: &RenderTask) -> Vec<String> {
    vec![
        "-b".to_string(),
        task.blend_file.display().to_string(),
        "-s".to_string(),
        task.start_frame.to_string(),
        "-e".to_string(),
        task.end_frame.to_string(),
        "-t".to_string(),
        task.threads.to_string(),
        "-x".to_string(),
        "1".to_string(),
        "-o".to_string(),
        "src/output".to_string(),
        "-a".to_string(),
    ]
}