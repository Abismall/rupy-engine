***"There are currently 5 games written in rust... and 50 game engines"***

## Acknowledgements

A special thanks to all the content creators, developers, and contributors who share their knowledge freely. While there are too many to list, I would like to mention a few:

- **[@TheCherno](https://www.youtube.com/@TheCherno)** – for excellent free tutorials on creating game engines.
- **[@TechWithTim](https://www.youtube.com/@TechWithTim)** – for a wide range of tutorials, covering topics from beginner to advanced.
- **[@Javidx9/Onelonecoder](https://www.onelonecoder.com)** – for offering programming and technology tutorials from a unique perspective.
```
rupyengine
├─ LICENSE
├─ py
│  ├─ Application
│  │  ├─ application.py
│  │  ├─ engine.py
│  │  ├─ menu_manager.py
│  │  ├─ process_manager.py
│  │  ├─ signal.py
│  │  └─ __init__.py
│  ├─ Error
│  │  ├─ base.py
│  │  ├─ category.py
│  │  ├─ messages.py
│  │  └─ __init__.py
│  ├─ init.py
│  ├─ Input
│  │  ├─ handlers.py
│  │  ├─ interfaces.py
│  │  ├─ manager.py
│  │  └─ __init__.py
│  ├─ main.py
│  ├─ main.spec
│  ├─ pyproject.toml
│  ├─ requirements.txt
│  ├─ UI
│  │  ├─ launcher.py
│  │  └─ scenes.py
│  └─ Utils
│     ├─ constants.py
│     ├─ environment.py
│     ├─ files.py
│     ├─ log.py
│     ├─ math.py
│     ├─ strings.py
│     ├─ validation.py
│     └─ __init__.py
├─ README.md
├─ ru
│  ├─ Cargo.lock
│  ├─ Cargo.toml
│  ├─ config.toml
│  ├─ run.bat
│  ├─ run.sh
│  ├─ src
│  │  ├─ app
│  │  │  ├─ app.rs
│  │  │  ├─ event_handler.rs
│  │  │  ├─ mod.rs
│  │  │  ├─ state.rs
│  │  │  └─ state_bit_flags.rs
│  │  ├─ assets
│  │  │  ├─ fonts
│  │  │  │  └─ font.ttf
│  │  │  ├─ res
│  │  │  │  ├─ cube.mtl
│  │  │  │  ├─ cube.obj
│  │  │  │  ├─ cube_diffuse.jpg
│  │  │  │  ├─ cube_normal.png
│  │  │  │  └─ pure-sky.hdr
│  │  │  ├─ scenes
│  │  │  ├─ shaders
│  │  │  │  ├─ equirectangular.wgsl
│  │  │  │  ├─ hdr.wgsl
│  │  │  │  ├─ light.wgsl
│  │  │  │  ├─ lit
│  │  │  │  │  └─ default
│  │  │  │  ├─ normal.wgsl
│  │  │  │  ├─ sky.wgsl
│  │  │  │  └─ unlit
│  │  │  │     └─ default
│  │  │  │        ├─ 2d_frag.wgsl
│  │  │  │        ├─ 2d_vert.wgsl
│  │  │  │        ├─ 3d_frag.wgsl
│  │  │  │        └─ 3d_vert.wgsl
│  │  │  └─ textures
│  │  │     └─ cube
│  │  ├─ audio
│  │  │  └─ mod.rs
│  │  ├─ bin
│  │  │  └─ resize_image.rs
│  │  ├─ camera
│  │  │  ├─ controller.rs
│  │  │  ├─ frustum.rs
│  │  │  ├─ mod.rs
│  │  │  └─ projection.rs
│  │  ├─ core
│  │  │  ├─ error.rs
│  │  │  ├─ files.rs
│  │  │  ├─ logging
│  │  │  │  ├─ factory.rs
│  │  │  │  ├─ level_filter.rs
│  │  │  │  └─ mod.rs
│  │  │  ├─ mod.rs
│  │  │  ├─ surface.rs
│  │  │  └─ worker.rs
│  │  ├─ ecs
│  │  │  ├─ cache.rs
│  │  │  ├─ components
│  │  │  │  ├─ component_data.rs
│  │  │  │  ├─ component_storage.rs
│  │  │  │  ├─ instance
│  │  │  │  │  ├─ mod.rs
│  │  │  │  │  └─ model.rs
│  │  │  │  ├─ material
│  │  │  │  │  ├─ manager.rs
│  │  │  │  │  ├─ mod.rs
│  │  │  │  │  └─ model.rs
│  │  │  │  ├─ mesh
│  │  │  │  │  ├─ manager.rs
│  │  │  │  │  ├─ mod.rs
│  │  │  │  │  └─ model.rs
│  │  │  │  ├─ mod.rs
│  │  │  │  ├─ model
│  │  │  │  │  ├─ manager.rs
│  │  │  │  │  ├─ mod.rs
│  │  │  │  │  └─ model.rs
│  │  │  │  └─ transform.rs
│  │  │  ├─ entity.rs
│  │  │  ├─ mod.rs
│  │  │  ├─ render_buffer.rs
│  │  │  ├─ render_context.rs
│  │  │  ├─ render_pass.rs
│  │  │  ├─ resources.rs
│  │  │  ├─ scene.rs
│  │  │  ├─ traits.rs
│  │  │  └─ world.rs
│  │  ├─ events
│  │  │  ├─ mod.rs
│  │  │  └─ proxy.rs
│  │  ├─ gpu
│  │  │  ├─ binding
│  │  │  │  ├─ camera.rs
│  │  │  │  ├─ environment.rs
│  │  │  │  ├─ equirect.rs
│  │  │  │  ├─ light.rs
│  │  │  │  ├─ mod.rs
│  │  │  │  └─ texture.rs
│  │  │  ├─ geometry
│  │  │  │  ├─ cube.rs
│  │  │  │  ├─ hexagon.rs
│  │  │  │  ├─ mod.rs
│  │  │  │  ├─ plane.rs
│  │  │  │  ├─ rectangle.rs
│  │  │  │  ├─ sphere.rs
│  │  │  │  └─ triangle.rs
│  │  │  ├─ mod.rs
│  │  │  ├─ model.rs
│  │  │  ├─ sampler.rs
│  │  │  └─ system
│  │  │     ├─ context.rs
│  │  │     ├─ global.rs
│  │  │     ├─ glyphon.rs
│  │  │     ├─ hdr.rs
│  │  │     ├─ mod.rs
│  │  │     └─ texture.rs
│  │  ├─ input
│  │  │  ├─ action.rs
│  │  │  └─ mod.rs
│  │  ├─ lib.rs
│  │  ├─ main.rs
│  │  ├─ math
│  │  │  ├─ mod.rs
│  │  │  └─ trigonometry.rs
│  │  ├─ pipeline
│  │  │  ├─ manager.rs
│  │  │  ├─ mod.rs
│  │  │  └─ setup.rs
│  │  ├─ shader
│  │  │  ├─ loader.rs
│  │  │  ├─ manager.rs
│  │  │  ├─ mod.rs
│  │  │  └─ module.rs
│  │  ├─ ui
│  │  │  └─ mod.rs
│  │  └─ utilities
│  │     ├─ constant.rs
│  │     ├─ frame.rs
│  │     ├─ helpers.rs
│  │     └─ mod.rs
│  └─ tests
│     └─ gpu.rs
├─ run_py.sh
└─ scripts
   ├─ is_pip_installed.sh
   └─ is_python_installed.sh

```