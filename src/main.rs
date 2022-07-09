use glam::vec3;
use macroquad::ui::{
    hash, root_ui,
    widgets::{self, Group},
};
use macroquad::{prelude::*, rand::gen_range};

const MOVE_SPEED: f32 = 0.1;
const LOOK_SPEED: f32 = 0.1;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Particle {
    Electron,
    Proton,
    Neutron,
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Charge {
    particle: Particle,
    mass: f32,
    pos: Vec3,
    vel: Vec3,
    acc: Vec3,
    trail: [Vec3; 50],
}

impl Charge {
    fn new(particle: Particle, pos: Option<Vec3>) -> Charge {
        let pos = match pos {
            Some(vec) => vec,
            None => gen_random_vector(-10., 10.),
        };

        Charge {
            particle,
            mass: get_mass(particle),
            pos,
            vel: vec3(0., 0., 0.),
            acc: vec3(0., 0., 0.),
            trail: [pos; 50],
        }
    }

    fn get_acc(&self, charge_vec: &Vec<Charge>) -> Vec3 {
        let k: f32 = 9f32 * 10f32.powf(9.);

        charge_vec
            .iter()
            .filter(|e| *e != self)
            .map(|e| {
                let unit_acc = |x: f32, y: f32| {
                    if (y - x).abs() != 0. {
                        k * get_charge(self.particle) * get_charge(e.particle)
                            / ((y - x).abs() * self.mass)
                    } else {
                        0.
                    }
                };
                // println!(
                //     "acc: {:?}, vec: {:?}, pos: {:?}",
                //     self.acc, self.vel, self.pos
                // );

                vec3(
                    unit_acc(self.pos.x, e.pos.x),
                    unit_acc(self.pos.y, e.pos.y),
                    unit_acc(self.pos.z, e.pos.z),
                )
            })
            .reduce(|e, acc| e + acc)
            .unwrap()
    }

    fn update(&mut self, charge_vec: &Vec<Charge>) {
        self.acc = self.get_acc(&charge_vec);
        self.vel += self.acc;
        self.pos += self.vel;
    }
}

fn get_charge(particle: Particle) -> f32 {
    let q: f32 = 1.6 * 10f32.powf(-19.);
    match particle {
        Particle::Electron => -q,
        Particle::Proton => q,
        Particle::Neutron => 0.,
    }
}

fn get_mass(particle: Particle) -> f32 {
    let em: f32 = 9.1 * 10f32.powf(-27.);
    let pm: f32 = 1.6 * 10f32.powf(-27.);
    match particle {
        Particle::Electron => em,
        _ => pm,
    }
}

fn gen_random_vector(start: f32, end: f32) -> Vec3 {
    let get_rand = || {
        let num = gen_range(start, end);
        if num == 0. {
            return start;
        }
        num
    };
    return vec3(get_rand(), get_rand(), get_rand());
}

fn conf() -> Conf {
    Conf {
        window_title: String::from("Rutherford Atomic Model"),
        window_width: 2360,
        window_height: 1368,
        // high_dpi: true,
        // fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut x = 0.0;
    let mut switch = false;
    let bounds = 8.0;

    let world_up = vec3(0.0, 1.0, 0.0);
    let mut yaw: f32 = 1.18;
    let mut pitch: f32 = 0.0;

    let mut front = vec3(
        yaw.cos() * pitch.cos(),
        pitch.sin(),
        yaw.sin() * pitch.cos(),
    )
    .normalize();
    let mut right = front.cross(world_up).normalize();
    let mut up;

    let mut position = vec3(0.0, 50.0, 0.0);
    let mut last_mouse_position: Vec2 = mouse_position().into();

    let mut grabbed = true;
    set_cursor_grab(grabbed);
    show_mouse(false);

    let mut charge_vec = vec![
        Charge::new(Particle::Electron, Some(vec3(200., 00., 0.))),
        Charge::new(Particle::Proton, Some(vec3(100., 0., 00.))),
    ];

    loop {
        let delta = get_frame_time();

        if is_key_pressed(KeyCode::Q) || is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Tab) {
            grabbed = !grabbed;
            set_cursor_grab(grabbed);
            show_mouse(!grabbed);
        }

        if is_key_down(KeyCode::W) {
            position += front * MOVE_SPEED;
        }
        if is_key_down(KeyCode::S) {
            position -= front * MOVE_SPEED;
        }
        if is_key_down(KeyCode::A) {
            position -= right * MOVE_SPEED;
        }
        if is_key_down(KeyCode::D) {
            position += right * MOVE_SPEED;
        }
        if is_key_down(KeyCode::Space) {
            position.y += MOVE_SPEED;
        }
        if is_key_down(KeyCode::LeftControl) {
            position.y -= MOVE_SPEED;
        }
        if is_key_pressed(KeyCode::Key1) {
            charge_vec.append(&mut vec![Charge::new(Particle::Electron, None)]);
        }
        if is_key_pressed(KeyCode::Key2) {
            charge_vec.append(&mut vec![Charge::new(Particle::Proton, None)]);
        }
        if is_key_pressed(KeyCode::Key3) {
            charge_vec.append(&mut vec![Charge::new(Particle::Neutron, None)]);
        }

        let mouse_position: Vec2 = mouse_position().into();
        let mouse_delta = mouse_position - last_mouse_position;
        last_mouse_position = mouse_position;

        yaw += mouse_delta.x * delta * LOOK_SPEED;
        pitch += mouse_delta.y * delta * -LOOK_SPEED;

        pitch = if pitch > 1.5 { 1.5 } else { pitch };
        pitch = if pitch < -1.5 { -1.5 } else { pitch };

        front = vec3(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos(),
        )
        .normalize()
            * 3.;

        right = front.cross(world_up).normalize() * 3.;
        up = right.cross(front).normalize() * 3.;

        x += if switch { 0.04 } else { -0.04 };
        if x >= bounds || x <= -bounds {
            switch = !switch;
        }

        println!("{:?}", charge_vec.iter().count());

        clear_background(BLACK);

        // !Implement UI
        // widgets::Window::new(hash!(), vec2(10., 10.), vec2(100., 100.))
        //     .label("A")
        //     .ui(&mut *root_ui(), |ui| {});

        // 3D
        set_camera(&Camera3D {
            position,
            up,
            target: position + front,
            ..Default::default()
        });

        draw_grid(2000, 10., BLACK, GRAY);

        let buffer = charge_vec.clone();
        for charge in &mut charge_vec {
            charge.update(&buffer);
            draw_sphere(charge.pos, 2., None, YELLOW);

            // for trail in charge.trail { //! Perf issues
            //     draw_sphere_wires(trail, 0.1, None, GREEN);
            // }
        }

        // Back to screen space, render some text
        set_default_camera();

        let fps = get_fps();
        draw_text(format!("{}", fps).as_str(), 10., 20., 30., GREEN);

        next_frame().await
    }
}
