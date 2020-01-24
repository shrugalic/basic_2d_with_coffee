mod colors;
mod coord_transform;

use crate::colors::*;
use crate::coord_transform::{Coord2, CoordinateTransformation};
use coffee::graphics::{Frame, Mesh, Window, WindowSettings};
use coffee::load::Task;
use coffee::{Game, Result, Timer};

fn main() -> Result<()> {
    MyStruct::run(WindowSettings {
        title: String::from("Basic coffee 2D"),
        size: (1280, 1280),
        resizable: true,
        fullscreen: false,
    })
}

impl Game for MyStruct {
    type Input = (); // No input data
    type LoadingScreen = (); // No loading screen

    fn load(_window: &Window) -> Task<MyStruct> {
        // Load your game assets here. Check out the `load` module!
        Task::new(|| MyStruct {
            points: vec![], // TODO create real object here
        })
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        let ct = CoordinateTransformation::from_points(frame.width(), frame.height(), &self.points);

        frame.clear(GREY);
        let mut mesh = Mesh::new();

        // TODO draw real stuff here
        for pos in &self.points {
            mesh.fill(ct.square_at(pos), YELLOW);
        }

        mesh.draw(&mut frame.as_target());
    }

    fn update(&mut self, _window: &Window) {
        // TODO update real stuff here
        std::thread::sleep(std::time::Duration::from_millis(1_000));
    }
}

struct MyStruct {
    points: Vec<Coord2>,
}

mod tests {
    #[test]
    fn always_successful() {
        assert_eq!(1 + 1, 2);
    }
}
