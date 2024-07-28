use std::net::IpAddr;
use std::path::PathBuf;

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
pub(crate) struct RenderTask {
    pub(crate) id: String,
    pub(crate) blend_file: PathBuf,
    pub(crate) start_frame: u32,
    pub(crate) end_frame: u32,
    pub(crate) render_engine: RenderEngine,
    pub(crate) threads: u32,
    pub(crate) status: TaskStatus,
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