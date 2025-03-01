use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

pub fn setup_clips(
    mut commands: Commands,
    mut library: ResMut<AnimationLibrary>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let spritesheet = Spritesheet::new(10, 7);
    commands.insert_resource(PlayerLayouts {
        cat: atlas_layouts.add(spritesheet.atlas_layout(76, 48)),
    });

    // Idle
    let idle_clip = Clip::from_frames(spritesheet.row(2));
    let idle_clip_id = library.register_clip(idle_clip);

    let idle_animation = Animation::from_clip(idle_clip_id);
    let idle_animation_id = library.register_animation(idle_animation);

    library
        .name_animation(idle_animation_id, "cat-idle")
        .unwrap();

    // Run
    let run_clip = Clip::from_frames(spritesheet.row(0));
    let run_clip_id = library.register_clip(run_clip);

    let run_animation = Animation::from_clip(run_clip_id);
    let run_animation_id = library.register_animation(run_animation);

    library.name_animation(run_animation_id, "cat-run").unwrap();
}

#[derive(Resource, Default)]
pub struct PlayerLayouts {
    pub cat: Handle<TextureAtlasLayout>,
}
