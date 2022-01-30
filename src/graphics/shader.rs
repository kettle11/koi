use crate::*;
use kgraphics::*;

use std::sync::mpsc;

#[derive(Clone)]
pub struct Shader {
    pub pipeline: Pipeline,
    #[cfg(feature = "xr")]
    pub multiview_pipeline: Option<Pipeline>, // pub transparent: bool,
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

impl ShaderAssetLoader {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        Self {
            sender: SyncGuard::new(sender),
            receiver: SyncGuard::new(receiver),
        }
    }
}

impl AssetLoader<Shader> for ShaderAssetLoader {
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

impl Shader {
    pub const UNLIT: Handle<Shader> = Handle::<Shader>::new_with_just_index(1);
    pub const PHYSICALLY_BASED: Handle<Shader> = Handle::<Shader>::new_with_just_index(2);
    pub const PHYSICALLY_BASED_TRANSPARENT: Handle<Shader> =
        Handle::<Shader>::new_with_just_index(3);
    pub const DEPTH_ONLY: Handle<Shader> = Handle::<Shader>::new_with_just_index(4);
    pub const UI: Handle<Shader> = Handle::<Shader>::new_with_just_index(5);
    pub const SKY_BOX: Handle<Shader> = Handle::<Shader>::new_with_just_index(6);
    pub const UNLIT_TRANSPARENT: Handle<Shader> = Handle::<Shader>::new_with_just_index(7);
    pub const PHYSICALLY_BASED_DOUBLE_SIDED: Handle<Shader> =
        Handle::<Shader>::new_with_just_index(8);
    pub const PHYSICALLY_BASED_TRANSPARENT_DOUBLE_SIDED: Handle<Shader> =
        Handle::<Shader>::new_with_just_index(9);
}

pub(crate) fn initialize_static_shaders(graphics: &mut Graphics, shaders: &mut Assets<Shader>) {
    shaders.add_and_leak(
        graphics
            .new_shader(
                include_str!("built_in_shaders/unlit.glsl"),
                // Render front and back as this may be used for sprites
                // that will be flipped.
                PipelineSettings {
                    faces_to_render: FacesToRender::FrontAndBack,
                    blending: None,
                    ..Default::default()
                },
            )
            .unwrap(),
        &Shader::UNLIT,
    );

    shaders.add_and_leak(
        graphics
            .new_shader(
                include_str!("built_in_shaders/physically_based.glsl"),
                PipelineSettings {
                    faces_to_render: FacesToRender::Front,
                    blending: None,
                    ..Default::default()
                },
            )
            .unwrap(),
        &Shader::PHYSICALLY_BASED,
    );

    shaders.add_and_leak(
        graphics
            .new_shader(
                include_str!("built_in_shaders/physically_based.glsl"),
                PipelineSettings {
                    faces_to_render: FacesToRender::Front,
                    blending: Some((BlendFactor::SourceAlpha, BlendFactor::OneMinusSourceAlpha)),
                    ..Default::default()
                },
            )
            .unwrap(),
        &Shader::PHYSICALLY_BASED_TRANSPARENT,
    );

    shaders.add_and_leak(
        graphics
            .new_shader(
                include_str!("built_in_shaders/depth_only.glsl"),
                PipelineSettings {
                    faces_to_render: FacesToRender::FrontAndBack,
                    ..Default::default()
                },
            )
            .unwrap(),
        &Shader::DEPTH_ONLY,
    );
    shaders.add_and_leak(
        graphics
            .new_shader(
                include_str!("built_in_shaders/unlit_ui.glsl"),
                PipelineSettings {
                    faces_to_render: FacesToRender::FrontAndBack,
                    blending: Some((BlendFactor::SourceAlpha, BlendFactor::OneMinusSourceAlpha)),
                    ..Default::default()
                },
            )
            .unwrap(),
        &Shader::UI,
    );

    shaders.add_and_leak(
        graphics
            .new_shader(
                include_str!("built_in_shaders/skybox.glsl"),
                PipelineSettings::default(),
            )
            .unwrap(),
        &Shader::SKY_BOX,
    );

    shaders.add_and_leak(
        graphics
            .new_shader(
                include_str!("built_in_shaders/unlit.glsl"),
                // Render front and back as this may be used for sprites
                // that will be flipped.
                PipelineSettings {
                    faces_to_render: FacesToRender::FrontAndBack,
                    blending: Some((BlendFactor::SourceAlpha, BlendFactor::OneMinusSourceAlpha)),
                    ..Default::default()
                },
            )
            .unwrap(),
        &Shader::UNLIT_TRANSPARENT,
    );

    shaders.add_and_leak(
        graphics
            .new_shader(
                include_str!("built_in_shaders/physically_based.glsl"),
                PipelineSettings {
                    faces_to_render: FacesToRender::FrontAndBack,
                    blending: None,
                    ..Default::default()
                },
            )
            .unwrap(),
        &Shader::PHYSICALLY_BASED_DOUBLE_SIDED,
    );

    shaders.add_and_leak(
        graphics
            .new_shader(
                include_str!("built_in_shaders/physically_based.glsl"),
                PipelineSettings {
                    faces_to_render: FacesToRender::FrontAndBack,
                    blending: Some((BlendFactor::SourceAlpha, BlendFactor::OneMinusSourceAlpha)),
                    ..Default::default()
                },
            )
            .unwrap(),
        &Shader::PHYSICALLY_BASED_TRANSPARENT_DOUBLE_SIDED,
    );
}
