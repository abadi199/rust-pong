use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::cgmath::{Matrix4, Vector3};
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, Event, PngFormat, Projection, Sprite, Texture, TextureHandle, VirtualKeyCode,
    WithSpriteRender,
};
use {
    Ball, Paddle, Side, ARENA_HEIGHT, ARENA_WIDTH, BALL_RADIUS, BALL_VELOCITY_X, BALL_VELOCITY_Y,
    PADDLE_HEIGHT, PADDLE_WIDTH, SPRITESHEET_SIZE,
};

pub struct Pong;

impl<'a, 'b> State<GameData<'a, 'b>> for Pong {
    fn handle_event(&mut self, _: StateData<GameData>, event: Event) -> Trans<GameData<'a, 'b>> {
        if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
            Trans::Quit
        } else {
            Trans::None
        }
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(&data.world);
        Trans::None
    }

    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
        let spritesheet = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                "texture/pong_spritesheet.png",
                PngFormat,
                Default::default(),
                (),
                &texture_storage,
            )
        };

        world.register::<Paddle>();

        initialise_paddle(world, spritesheet.clone());
        initialise_ball(world, spritesheet);
        initialise_camera(world);
    }
}

fn initialise_camera(world: &mut World) {
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            ARENA_HEIGHT,
            0.0,
        )))
        .with(GlobalTransform(
            Matrix4::from_translation(Vector3::new(0.0, 0.0, 1.0)).into(),
        ))
        .build();
}

fn initialise_paddle(world: &mut World, spritesheet: TextureHandle) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    let y = ARENA_HEIGHT / 2.0;
    left_transform.translation = Vector3::new(PADDLE_WIDTH * 0.5, y, 0.0);
    right_transform.translation = Vector3::new(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);

    let sprite = Sprite {
        left: 0.0,
        right: PADDLE_WIDTH,
        top: 0.0,
        bottom: PADDLE_HEIGHT,
    };

    world
        .create_entity()
        .with_sprite(&sprite, spritesheet.clone(), SPRITESHEET_SIZE)
        .expect("Failed to add sprite render on left paddle")
        .with(Paddle::new(Side::Left))
        .with(GlobalTransform::default())
        .with(left_transform)
        .build();

    world
        .create_entity()
        .with_sprite(&sprite, spritesheet.clone(), SPRITESHEET_SIZE)
        .expect("Failed to add sprite render on left paddle")
        .with(Paddle::new(Side::Left))
        .with(Paddle::new(Side::Right))
        .with(GlobalTransform::default())
        .with(right_transform)
        .build();
}

fn initialise_ball(world: &mut World, spritesheet: TextureHandle) {
    // Create the translation.
    let mut local_transform = Transform::default();
    local_transform.translation = Vector3::new(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);

    // Create the sprite for the ball.
    let sprite = Sprite {
        left: PADDLE_WIDTH,
        right: SPRITESHEET_SIZE.0,
        top: 0.0,
        bottom: BALL_RADIUS * 2.0,
    };

    world
        .create_entity()
        .with_sprite(&sprite, spritesheet, SPRITESHEET_SIZE)
        .expect("Error creating SpriteRender for ball")
        .with(Ball {
            radius: BALL_RADIUS,
            velocity: [BALL_VELOCITY_X, BALL_VELOCITY_Y],
        })
        .with(local_transform)
        .with(GlobalTransform::default())
        .build();
}
