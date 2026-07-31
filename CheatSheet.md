
(bevy::prelude)
Enum SpriteImageMode

(bevy::prelude)
Struct Transform 

(bevy::prelude)
Struct GlobalTransform


OBS: 
> ### [`   Transform`](https://docs.rs/bevy/latest/bevy/prelude/struct.Transform.html "struct bevy::prelude::Transform") vs. [`GlobalTransform`](https://docs.rs/bevy/latest/bevy/prelude/struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform")


(bevy::prelude)
Struct TransformHelper

(bevy::math::bounding)
Trait IntersectsVolume

(bevy::prelude)
Struct Or

(bevy)
Create gizmos
> The **`bevy_gizmos`** crate provides an **immediate mode drawing API** for the Bevy game engine. Its primary purpose is **visual debugging**, allowing developers to easily draw lines, shapes, and other debug visuals directly in their 3D or 2D scenes.

OBS:
> Create Avian3d


OBS: 
> Why Native Bevy Games Get Full GPU Performance

The performance gain is tied to the **hardware (your GPU)**, not the browser. When you build a desktop game using **Rust** and **Bevy**, the compilation and execution happen like this:

- **Direct Hardware Access**: Bevy uses a Rust library called `wgpu` underneath. On desktop, `wgpu` bypasses the browser completely. [[1](https://www.rustadventure.dev/webgpu-is-coming-in-bevy-0-11), [2](https://rustify.rs/articles/rust-gpu-computing-wgpu-2026), [3](https://surma.dev/things/webgpu/)]
- **Native Translation**: When your game starts, `wgpu` automatically translates your WGSL shaders into the native language of your operating system's graphics driver.
    - On **Windows**, it translates WGSL to **HLSL** (DirectX 12).
    - On **macOS/iOS**, it translates WGSL to **MSL** (Metal).
    - On **Linux/Android**, it translates WGSL to **SPIR-V** (Vulkan). [[1](https://www.doc.ic.ac.uk/~afd/masters_theses/Mohsin.pdf), [2](https://github.com/wgslfuzz/darthshader), [3](https://rust-gpu.github.io/blog/2025/07/25/rust-on-every-gpu/), [4](https://news.ycombinator.com/item?id=27195704), [5](https://streamhpc.com/blog/2015-05-21/8-reasons-why-spir-v-makes-a-big-difference/)]