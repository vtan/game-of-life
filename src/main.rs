use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

const WINDOW_SIZE: u32 = 1024;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video
        .window("game of life", WINDOW_SIZE, WINDOW_SIZE)
        .build()
        .unwrap();
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .unwrap();

    let cell_pixels = WINDOW_SIZE / (WORLD_SIZE as u32);

    let mut world = World::new();

    let mut event_pump = sdl.event_pump().unwrap();
    let mut running = true;
    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => running = false,

                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    x,
                    y,
                    ..
                } => {
                    let i = x / (cell_pixels as i32);
                    let j = y / (cell_pixels as i32);
                    if i >= 0 && i < (WINDOW_SIZE as i32) && j >= 0 && j < (WINDOW_SIZE as i32) {
                        let cur = world.get_cell(i as usize, j as usize);
                        world.set_cell(i as usize, j as usize, !cur);
                    }
                }

                Event::KeyDown {
                    scancode: Some(Scancode::C),
                    repeat: false,
                    ..
                } => {
                    world = World::new();
                }

                Event::KeyDown {
                    scancode: Some(Scancode::R),
                    repeat: false,
                    ..
                } => {
                    let rng = fastrand::Rng::new();
                    world = World::new();
                    let m = 16;
                    let n = 16;
                    for j in 0..n {
                        for i in 0..m {
                            if rng.f32() >= 0.5 {
                                world.set_cell(
                                    i + (WORLD_SIZE - n) / 2,
                                    j + (WORLD_SIZE - n) / 2,
                                    true,
                                );
                            }
                        }
                    }
                }

                Event::KeyDown {
                    scancode: Some(Scancode::Space),
                    ..
                } => {
                    world.step();
                }

                _ => {}
            }
        }

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        canvas.set_draw_color(Color::YELLOW);
        for i in 0..WORLD_SIZE {
            for j in 0..WORLD_SIZE {
                if world.get_cell(i, j) {
                    let x = (i as u32 * cell_pixels) as i32;
                    let y = (j as u32 * cell_pixels) as i32;
                    canvas
                        .fill_rect(Some(Rect::new(x, y, cell_pixels, cell_pixels)))
                        .unwrap();
                }
            }
        }

        canvas.set_draw_color(Color::GREEN);
        {
            let xy = ((WORLD_SIZE as i32 - 16) / 2) * cell_pixels as i32;
            let wh = 16 * cell_pixels;
            canvas.draw_rect(Rect::new(xy, xy, wh, wh)).unwrap();
        }

        canvas.set_draw_color(Color::MAGENTA);
        {
            let mouse = event_pump.mouse_state();
            let x = mouse.x() / (cell_pixels as i32) * (cell_pixels as i32);
            let y = mouse.y() / (cell_pixels as i32) * (cell_pixels as i32);
            canvas
                .draw_rect(Rect::new(x, y, cell_pixels, cell_pixels))
                .unwrap();
        }

        canvas.present();
    }
}

const WORLD_SIZE: usize = 256;

struct World {
    current: Box<[bool; WORLD_SIZE * WORLD_SIZE]>,
    next: Box<[bool; WORLD_SIZE * WORLD_SIZE]>,
}

impl World {
    pub fn new() -> Self {
        Self {
            current: Box::new([false; WORLD_SIZE * WORLD_SIZE]),
            next: Box::new([false; WORLD_SIZE * WORLD_SIZE]),
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> bool {
        self.current[Self::index(x, y)]
    }

    pub fn set_cell(&mut self, x: usize, y: usize, alive: bool) {
        self.current[Self::index(x, y)] = alive
    }

    pub fn step(&mut self) {
        for cell in self.next.iter_mut() {
            *cell = false;
        }

        for i in 1..(WORLD_SIZE - 1) {
            for j in 1..(WORLD_SIZE - 1) {
                let mut count = 0_i32;
                count += self.get_cell(i - 1, j - 1) as i32;
                count += self.get_cell(i - 1, j) as i32;
                count += self.get_cell(i - 1, j + 1) as i32;
                count += self.get_cell(i, j - 1) as i32;
                count += self.get_cell(i, j + 1) as i32;
                count += self.get_cell(i + 1, j - 1) as i32;
                count += self.get_cell(i + 1, j) as i32;
                count += self.get_cell(i + 1, j + 1) as i32;
                match count {
                    2 => self.next[Self::index(i, j)] = self.get_cell(i, j),
                    3 => self.next[Self::index(i, j)] = true,
                    _ => {}
                }
            }
        }
        std::mem::swap(&mut self.current, &mut self.next);
    }

    fn index(x: usize, y: usize) -> usize {
        assert!(x < WORLD_SIZE);
        assert!(y < WORLD_SIZE);
        x + y * WORLD_SIZE
    }
}
