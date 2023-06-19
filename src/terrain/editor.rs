use bevy::prelude::*;

pub(crate) struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}

/*
    IoTaskPool::get()
    .spawn(async move {
        File::create(format!("assets/{PATH}"))
        .and_then(|mut file| file.write(serialized_scene.as_bytes()))
        .expect("Error while writing to file");
    })
    .detach();
 */