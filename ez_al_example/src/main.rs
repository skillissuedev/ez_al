use std::env;
use ez_al::{WavAsset, SoundSource, SoundSourceType};
use three_d::*;

pub fn main() {
    let window = Window::new(WindowSettings {
        title: "Shapes!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut gui = GUI::new(&context);

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(5.0, 2.0, 2.5),
        vec3(0.0, 0.0, -0.5),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    let cube = Gm::new(
        Mesh::new(&context, &CpuMesh::cube()),
        PhysicalMaterial::new_opaque(
            &context,
            &CpuMaterial {
                albedo: Srgba {
                    r: 0,
                    g: 0,
                    b: 255,
                    a: 255,
                },
                ..Default::default()
            },
        ),
    );
    let light0 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(0.0, -0.5, -0.5));


    // Initializing ez_al
    ez_al::init().expect("Failed to init ez_al!");
    // Creating an asset
    let asset = WavAsset::from_wav(&get_full_asset_path("sound.wav"))
        .expect("Failed to load .wav file! Make sure that it's placed in the same directory as executable and named 'sound.wav'");
    // Creating sources
    let mut pos_source = SoundSource::new(&asset, SoundSourceType::Positional)
        .expect("Failed to create a positional sound source");
    let mut simple_source = SoundSource::new(&asset, SoundSourceType::Simple)
        .expect("Failed to create a simple sound source");

    let _ = pos_source.update([0.0, 0.0, 0.0]);
    let _ = pos_source.set_max_distance(30.0);


    window.render_loop(move |mut frame_input| {
        gui.update(&mut frame_input.events, frame_input.accumulated_time, frame_input.viewport, frame_input.device_pixel_ratio, 
            |gui_context| {
                egui::Window::new("").show(gui_context, |ui| {
                    ui.label("To move camera use LMB and mouse wheel");
                    ui.label("To play sound using positional source press E");
                    ui.label("To play sound using simple source press F");
                });
            }
        );

        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);
        for ev in &frame_input.events {
            match ev {
                Event::KeyPress { kind, modifiers: _, handled: _ } => {
                    if let Key::E = kind {
                        // Playing sound using positional source if key E is pressed
                        pos_source.play_sound();
                    }
                    if let Key::F = kind {
                        // Playing sound using simple source if key F is pressed
                        simple_source.play_sound();
                    }
                },
                _ => ()
            }
        }

        let cam_pos = camera.position();
        let cam_at = camera.view_direction();
        let cam_up = camera.view_direction();

        // Setting listener position
        ez_al::set_listener_transform([cam_pos.x, cam_pos.y, cam_pos.z], cam_at.into(), [cam_up.x, cam_up.y, cam_up.z]);

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(
                &camera,
                &cube,
                &[&light0],
            ).write(|| gui.render());

        FrameOutput::default()
    });
}


pub fn get_full_asset_path(path: &str) -> String {
    let mut exec_path: String = "".to_string();

    match env::current_exe() {
        Ok(exe_path) => {
            let executable_path = exe_path.to_str();
            match executable_path {
                Some(executable_path_string) => exec_path = executable_path_string.to_owned(), //println!("Path of this executable is: {}", executable_path_string.to_owned() + "/" + path),
                None => panic!("Getting current exe path error!"),
            }
        }
        Err(_e) => panic!("Getting current exe path error!"),
    };

    let full_exec_path_splitted: Vec<&str> = exec_path.split("/").collect();

    let mut full_path: String = "".to_string();

    for i in 0..full_exec_path_splitted.len() - 1 {
        full_path += full_exec_path_splitted[i];
        full_path += "/";
    }

    full_path += path;

    if cfg!(windows) {
        return full_path.replace("/", r"\");
    }

    full_path
}

