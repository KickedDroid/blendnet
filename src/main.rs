mod lib;

use std::{error::Error, path::PathBuf, process::Command, str::FromStr};

use lib::*;

fn construct_blender_command(task: &RenderTask) -> Vec<String> {
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
        "src/output/".to_string(),
        "-a".to_string(),
    ]
}

fn execute_render(task: &RenderTask) -> Result<(), Box<dyn Error>> {
    let args = construct_blender_command(task);
    
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

fn main() {
    let rendertask = RenderTask {
        id: "Device 1".to_string(),
        blend_file: PathBuf::from_str("overwerk.blend").unwrap(),
        start_frame: 0,
        end_frame: 1,
        render_engine: RenderEngine::Cycles,
        threads: 1,
        status: TaskStatus::Queued,
    };

    let args = construct_blender_command(&rendertask);
    
    args.iter().for_each(|i| println!("{}", i));


    execute_render(&rendertask).unwrap();
    
}

