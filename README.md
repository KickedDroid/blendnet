
# Blendnet 

*== Under heavy construction ==*

A customizable distributed render swarm for blender using `iroh-net` under the hood.  

Features:
- Connect using `iroh-relay`
- Recieve Renders to specific OutDir
- Ephemeral Client - Terminates after Render. ( Intended - to save costs ) 

In Progress:
- Client Daemon
- Private Pkarr discovery
- Gossip Subscriptions for workload propagation.
- Custom Config files
- Customized Load Balancing
- Dockerfile
- Easy Deplyment on VPS
- Blender Sandboxing
- Fail to destroy. ( Dstroys Artifacts and terminates for self presvervation ) 


# Usage

```
blendnet server --input <Blender_File> --output <OUT_DIR>
```

Client 

```
./blendnet client --node-id <NODE_ID> --relay-url <RELAY_URL> --addrs <Addrs>
```


Example output 

```
Connected to Blender Render server
Received 1048576 bytes. Total: 1048576 bytes
Received 201424 bytes. Total: 1250000 bytes
Received 28400 bytes. Total: 1278400 bytes
Received 170400 bytes. Total: 1448800 bytes
Received 28400 bytes. Total: 1477200 bytes
Received 28400 bytes. Total: 1505600 bytes
Received 28390 bytes. Total: 1533990 bytes
Received 28400 bytes. Total: 1562390 bytes
Received 28400 bytes. Total: 1590790 bytes
Received 28400 bytes. Total: 1619190 bytes
Received 28400 bytes. Total: 1647590 bytes
Received 28400 bytes. Total: 1675990 bytes
Received 28400 bytes. Total: 1704390 bytes
Received 28400 bytes. Total: 1732790 bytes
Received 28400 bytes. Total: 1761190 bytes
Received 56800 bytes. Total: 1817990 bytes
Received 28400 bytes. Total: 1846390 bytes
Received 28400 bytes. Total: 1874790 bytes
Received 28390 bytes. Total: 1903180 bytes
Received 56800 bytes. Total: 1959980 bytes
Received 28400 bytes. Total: 1988380 bytes
Received 14200 bytes. Total: 2002580 bytes
Received 28400 bytes. Total: 2030980 bytes
Received 14200 bytes. Total: 2045180 bytes
Received 14200 bytes. Total: 2059380 bytes
Received 14200 bytes. Total: 2073580 bytes
Received 28400 bytes. Total: 2101980 bytes
Received 28400 bytes. Total: 2130380 bytes
Received 14200 bytes. Total: 2144580 bytes
Received 14200 bytes. Total: 2158780 bytes
Received 14200 bytes. Total: 2172980 bytes
Received 14200 bytes. Total: 2187180 bytes
Received 28390 bytes. Total: 2215570 bytes
Received 14200 bytes. Total: 2229770 bytes
Received 14200 bytes. Total: 2243970 bytes
Received 14200 bytes. Total: 2258170 bytes
Received 14200 bytes. Total: 2272370 bytes
Received 14200 bytes. Total: 2286570 bytes
Received 14200 bytes. Total: 2300770 bytes
Received 14200 bytes. Total: 2314970 bytes
Received 14200 bytes. Total: 2329170 bytes
Received 28400 bytes. Total: 2357570 bytes
Received 14188 bytes. Total: 2371758 bytes
Received 14200 bytes. Total: 2385958 bytes
Received 14200 bytes. Total: 2400158 bytes
Received 14200 bytes. Total: 2414358 bytes
Received 14200 bytes. Total: 2428558 bytes
Received 14200 bytes. Total: 2442758 bytes
Received 14200 bytes. Total: 2456958 bytes
Received 14200 bytes. Total: 2471158 bytes
Received 14200 bytes. Total: 2485358 bytes
Received 14200 bytes. Total: 2499558 bytes
Received 28400 bytes. Total: 2527958 bytes
Received 22565 bytes. Total: 2550523 bytes
Blender file received and saved. Total bytes: 2550523
Created RenderTask: RenderTask { id: "Task", blend_file: "received_blend_file_0.blend", start_frame: 0, end_frame: 1, render_engine: Cycles, threads: 4 }

Render complete. Output: Blender 4.2.1 LTS (hash 396f546c9d82 built 2024-08-19 23:32:23)
Read blend: "/home/zdroid/Documents/Projects/blendnet/received_blend_file_0.blend"
Fra:0 Mem:128.76M (Peak 135.62M) | Time:00:00.21 | Rendering 1 / 64 samples
Fra:0 Mem:128.76M (Peak 135.62M) | Time:00:00.27 | Rendering 25 / 64 samples
Fra:0 Mem:128.76M (Peak 135.62M) | Time:00:00.34 | Rendering 50 / 64 samples
Fra:0 Mem:128.76M (Peak 135.62M) | Time:00:00.38 | Rendering 64 / 64 samples
Fra:0 Mem:93.93M (Peak 185.01M) | Time:00:00.44 | Compositing
Fra:0 Mem:93.94M (Peak 185.01M) | Time:00:00.44 | Compositing | Initializing execution
Fra:0 Mem:93.94M (Peak 185.01M) | Time:00:00.44 | Compositing | Operation 2-10
Fra:0 Mem:93.94M (Peak 185.01M) | Time:00:00.44 | Compositing | Operation 3-10
Fra:0 Mem:150.25M (Peak 185.01M) | Time:00:00.45 | Compositing | Operation 4-10
Fra:0 Mem:150.25M (Peak 206.50M) | Time:00:00.76 | Compositing | Operation 5-10
Fra:0 Mem:206.50M (Peak 206.50M) | Time:00:00.77 | Compositing | Operation 6-10
Fra:0 Mem:206.50M (Peak 262.75M) | Time:00:01.00 | Compositing | Operation 7-10
Fra:0 Mem:206.50M (Peak 262.75M) | Time:00:01.00 | Compositing | Operation 8-10
Fra:0 Mem:206.50M (Peak 262.75M) | Time:00:01.00 | Compositing | Operation 9-10
Fra:0 Mem:150.25M (Peak 262.75M) | Time:00:01.01 | Compositing | Operation 10-10
Fra:0 Mem:150.25M (Peak 262.75M) | Time:00:01.01 | Compositing | Operation 11-10
Saved: 'output/0000.png'
Time: 00:01.69 (Saving: 00:00.67)

Fra:1 Mem:128.77M (Peak 276.76M) | Time:00:00.00 | Rendering 1 / 64 samples
Fra:1 Mem:128.77M (Peak 276.76M) | Time:00:00.06 | Rendering 25 / 64 samples
Fra:1 Mem:128.77M (Peak 276.76M) | Time:00:00.13 | Rendering 50 / 64 samples
Fra:1 Mem:128.77M (Peak 276.76M) | Time:00:00.17 | Rendering 64 / 64 samples
Fra:1 Mem:93.98M (Peak 276.76M) | Time:00:00.23 | Compositing
Fra:1 Mem:93.98M (Peak 276.76M) | Time:00:00.23 | Compositing | Initializing execution
Fra:1 Mem:93.98M (Peak 276.76M) | Time:00:00.23 | Compositing | Operation 2-10
Fra:1 Mem:93.98M (Peak 276.76M) | Time:00:00.23 | Compositing | Operation 3-10
Fra:1 Mem:150.30M (Peak 276.76M) | Time:00:00.23 | Compositing | Operation 4-10
Fra:1 Mem:150.30M (Peak 276.76M) | Time:00:00.52 | Compositing | Operation 5-10
Fra:1 Mem:206.55M (Peak 276.76M) | Time:00:00.53 | Compositing | Operation 6-10
Fra:1 Mem:206.55M (Peak 276.76M) | Time:00:00.76 | Compositing | Operation 7-10
Fra:1 Mem:206.55M (Peak 276.76M) | Time:00:00.76 | Compositing | Operation 8-10
Fra:1 Mem:206.55M (Peak 276.76M) | Time:00:00.76 | Compositing | Operation 9-10
Fra:1 Mem:150.30M (Peak 276.76M) | Time:00:00.77 | Compositing | Operation 10-10
Fra:1 Mem:150.30M (Peak 276.76M) | Time:00:00.77 | Compositing | Operation 11-10
Saved: 'output/0001.png'
Time: 00:01.45 (Saving: 00:00.67)


Blender quit

Sending file: ./output/0000.png
Sent 1048576 / 17596444 bytes of ./output/0000.png
Sent 2097152 / 17596444 bytes of ./output/0000.png
Sent 3145728 / 17596444 bytes of ./output/0000.png
Sent 4194304 / 17596444 bytes of ./output/0000.png
Sent 5242880 / 17596444 bytes of ./output/0000.png
Sent 6291456 / 17596444 bytes of ./output/0000.png
Sent 7340032 / 17596444 bytes of ./output/0000.png
Sent 8388608 / 17596444 bytes of ./output/0000.png
Sent 9437184 / 17596444 bytes of ./output/0000.png
Sent 10485760 / 17596444 bytes of ./output/0000.png
Sent 11534336 / 17596444 bytes of ./output/0000.png
Sent 12582912 / 17596444 bytes of ./output/0000.png
Sent 13631488 / 17596444 bytes of ./output/0000.png
Sent 14680064 / 17596444 bytes of ./output/0000.png
Sent 15728640 / 17596444 bytes of ./output/0000.png
Sent 16777216 / 17596444 bytes of ./output/0000.png
Sent 17596444 / 17596444 bytes of ./output/0000.png
Sending file: ./output/0001.png
Sent 1048576 / 17601172 bytes of ./output/0001.png
Sent 2097152 / 17601172 bytes of ./output/0001.png
Sent 3145728 / 17601172 bytes of ./output/0001.png
Sent 4194304 / 17601172 bytes of ./output/0001.png
Sent 5242880 / 17601172 bytes of ./output/0001.png
Sent 6291456 / 17601172 bytes of ./output/0001.png
Sent 7340032 / 17601172 bytes of ./output/0001.png
Sent 8388608 / 17601172 bytes of ./output/0001.png
Sent 9437184 / 17601172 bytes of ./output/0001.png
Sent 10485760 / 17601172 bytes of ./output/0001.png
Sent 11534336 / 17601172 bytes of ./output/0001.png
Sent 12582912 / 17601172 bytes of ./output/0001.png
Sent 13631488 / 17601172 bytes of ./output/0001.png
Sent 14680064 / 17601172 bytes of ./output/0001.png
Sent 15728640 / 17601172 bytes of ./output/0001.png
Sent 16777216 / 17601172 bytes of ./output/0001.png
Sent 17601172 / 17601172 bytes of ./output/0001.png
Rendering complete!
All render results sent.
```
