mod lib;

use std::{error::Error, path::PathBuf, process::Command, str::FromStr};

use lib::*;


fn main() {
    let rendertask = RenderTask {
        id: "1".to_string(),
        blend_file: PathBuf::from_str("overwerk.blend").unwrap(),
        start_frame: 0,
        end_frame: 1,
        render_engine: RenderEngine::Cycles,
        threads: 1,
    };

    execute_render(&rendertask).unwrap();
    
}
