
# Blendnet 

*== Under heavy construction ==*

A customizable distributed render swarm for blender using `iroh-net` under the hood. 

In Progress:
- Custom Config files
- Customized Load Balancing
- 




```rust
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
```


Example output 

```
Render complete. Output: Blender 4.2.0 (hash a51f293548ad built 2024-07-16 06:27:02)
Read blend: "/home/zdroid/Documents/Programming/blendnet/overwerk.blend"
Fra:0 Mem:93.78M (Peak 184.86M) | Time:00:00.77 | Compositing
Fra:0 Mem:93.79M (Peak 184.86M) | Time:00:00.77 | Compositing | Initializing execution
Fra:0 Mem:93.79M (Peak 184.86M) | Time:00:00.77 | Compositing | Operation 2-10
Fra:0 Mem:93.79M (Peak 184.86M) | Time:00:00.77 | Compositing | Operation 3-10
Fra:0 Mem:150.10M (Peak 184.86M) | Time:00:00.78 | Compositing | Operation 4-10
Fra:0 Mem:150.10M (Peak 206.35M) | Time:00:01.32 | Compositing | Operation 5-10
Fra:0 Mem:206.35M (Peak 206.35M) | Time:00:01.37 | Compositing | Operation 6-10
Fra:0 Mem:206.35M (Peak 262.60M) | Time:00:02.26 | Compositing | Operation 7-10
Fra:0 Mem:206.35M (Peak 262.60M) | Time:00:02.26 | Compositing | Operation 8-10
Fra:0 Mem:206.35M (Peak 262.60M) | Time:00:02.26 | Compositing | Operation 9-10
Fra:0 Mem:150.10M (Peak 262.60M) | Time:00:02.28 | Compositing | Operation 10-10
Fra:0 Mem:150.10M (Peak 262.60M) | Time:00:02.28 | Compositing | Operation 11-10
Saved: 'src/output0000.png'
Time: 00:03.38 (Saving: 00:01.09)

Fra:1 Mem:93.83M (Peak 276.61M) | Time:00:00.30 | Compositing
Fra:1 Mem:93.83M (Peak 276.61M) | Time:00:00.30 | Compositing | Initializing execution
Fra:1 Mem:93.83M (Peak 276.61M) | Time:00:00.30 | Compositing | Operation 2-10
Fra:1 Mem:93.83M (Peak 276.61M) | Time:00:00.30 | Compositing | Operation 3-10
Fra:1 Mem:150.14M (Peak 276.61M) | Time:00:00.31 | Compositing | Operation 4-10
Fra:1 Mem:150.14M (Peak 276.61M) | Time:00:00.83 | Compositing | Operation 5-10
Fra:1 Mem:206.39M (Peak 276.61M) | Time:00:00.88 | Compositing | Operation 6-10
Fra:1 Mem:206.39M (Peak 276.61M) | Time:00:01.73 | Compositing | Operation 7-10
Fra:1 Mem:206.39M (Peak 276.61M) | Time:00:01.73 | Compositing | Operation 8-10
Fra:1 Mem:206.39M (Peak 276.61M) | Time:00:01.73 | Compositing | Operation 9-10
Fra:1 Mem:150.14M (Peak 276.61M) | Time:00:01.75 | Compositing | Operation 10-10
Fra:1 Mem:150.15M (Peak 276.61M) | Time:00:01.76 | Compositing | Operation 11-10
Saved: 'src/output0001.png'
Time: 00:02.64 (Saving: 00:00.88)
```
