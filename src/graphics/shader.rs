use crate::*;
use kgraphics::*;

use std::sync::mpsc;

#[derive(Clone)]
pub struct Shader {
    pub pipeline: Pipeline,
    // pub transparent: bool,
}

/// A system that loads shaders onto the GPU
pub(crate) fn load_shaders(shaders: &mut Assets<Shader>, graphics: &mut Graphics) {
    // A Vec doesn't need to be allocated here.
    // This is just a way to not borrow the ShaderAssetLoader and Assets<Shader> at
    // the same time.
    let messages: Vec<ShaderLoadMessage> =
        shaders.asset_loader.receiver.inner().try_iter().collect();
    for message in messages.into_iter() {
        let shader = graphics
            .new_shader(&message.source, message.pipeline_settings)
            .unwrap();
        shaders.replace_placeholder(&message.handle, shader);
    }
}
pub struct ShaderAssetLoader {
    sender: SyncGuard<mpsc::Sender<ShaderLoadMessage>>,
    receiver: SyncGuard<mpsc::Receiver<ShaderLoadMessage>>,
}

struct ShaderLoadMessage {
    handle: Handle<Shader>,
    source: String,
    pipeline_settings: PipelineSettings,
}

impl AssetLoader<Shader> for ShaderAssetLoader {
    fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        Self {
            sender: SyncGuard::new(sender),
            receiver: SyncGuard::new(receiver),
        }
    }

    fn load_with_options(
        &mut self,
        path: &str,
        handle: Handle<Shader>,
        pipeline_settings: <Shader as LoadableAssetTrait>::Options,
    ) {
        let path = path.to_owned();
        let sender = self.sender.inner().clone();

        ktasks::spawn(async move {
            let source = std::fs::read_to_string(path).unwrap();
            let _ = sender.send(ShaderLoadMessage {
                handle,
                source,
                pipeline_settings,
            });
        })
        .run();
    }
}
impl LoadableAssetTrait for Shader {
    type AssetLoader = ShaderAssetLoader;
    type Options = PipelineSettings;
}

pub const UNLIT_SHADER: Handle<Shader> = Handle::<Shader>::new_with_just_index(1);
/*
pub const PHYSICALLY_BASED_TRANSPARENT_SHADER: Handle<Shader> =
    Handle::<Shader>::new_with_just_index(2);
pub const DEPTH_ONLY_SHADER: Handle<Shader> = Handle::<Shader>::new_with_just_index(3);
*/
pub fn initialize_static_shaders(graphics: &mut Graphics, shaders: &mut Assets<Shader>) {
    // Perhaps there should be a separate unblended unlit shader?
    shaders.add_and_leak(
        graphics
            .new_shader(
                include_str!("default_shaders/unlit.glsl"),
                // Render front and back as this may be used for sprites
                // that will be flipped.
                PipelineSettings {
                    faces_to_render: FacesToRender::FrontAndBack,
                    blending: Some((BlendFactor::SourceAlpha, BlendFactor::OneMinusSourceAlpha)),
                },
            )
            .unwrap(),
        &UNLIT_SHADER,
    );

    /*
    shaders.add_and_leak(
        graphics
            .new_shader(
                include_str!("shaders/physically_based.glsl"),
                FacesToRender::Front,
                Some((BlendFactor::SourceAlpha, BlendFactor::OneMinusSourceAlpha)),
            )
            .unwrap(),
        &PHYSICALLY_BASED_TRANSPARENT_SHADER,
    );
    shaders.add_and_leak(
        graphics
            .new_shader(
                include_str!("shaders/depth_only.glsl"),
                FacesToRender::Front,
                None,
            )
            .unwrap(),
        &DEPTH_ONLY_SHADER,
    );
    */
}
