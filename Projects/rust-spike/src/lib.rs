use godot::prelude::*;
use godot::classes::Sprite2D;
use godot::classes::ISprite2D;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
    speed: f64,
    angular_speed: f64,

    base: Base<Sprite2D>
}
#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("Hello, world!"); //Prints to the Godot console
        
        Self {
            speed: 400.,
            angular_speed: std::f64::consts::PI,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let radians = (self.angular_speed * delta) as f32;
        self.base_mut().rotate(radians);
        //rotate method requires f32,
        //therefore we convert 'self.angular_speed * delta' which is a f64 to a f32
        let rotation = self.base().get_rotation();
        let velocity = Vector2::UP.rotated(rotation) * self.speed as f32;
        self.base_mut().translate(velocity * delta as f32);
    }
}

#[gdextension]
unsafe impl ExtensionLibrary for Player {}
