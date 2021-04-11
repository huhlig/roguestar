use bevy::asset::{Assets, Handle};
use bevy::core::Timer;
use bevy::ecs::{Query, Res};
use bevy::sprite::TextureAtlas;
use bevy::utils::HashMap;

pub struct SpriteCatalog {
    names: HashMap<String, usize>,
    collections: Vec<SpriteCollection>,
}

pub struct SpriteCollection {
    names: HashMap<String, usize>,
    animations: Vec<SpriteAnimation>,
}
pub struct SpriteAnimation {
    frames: Vec<SpriteFrame>,
}

pub struct SpriteFrame {
    texture_atlas: Handle<TextureAtlas>,
    texture_index: u32,
    duration: f32,
}

pub struct SpriteAnimator {
    current_collection: Option<usize>,
    current_animation: usize,
    current_frame: usize,
    current_timer: Timer,
}

impl SpriteAnimationBuilder {
    /// Begin building a new Sprite Animation
    pub fn build(atlas_handle: Handle<TextureAtlas>) -> SpriteAnimationBuilder {
        SpriteAnimationBuilder {
            frames: Vec::new(),
            atlas_handle,
            atlas_index: 0,
            duration: 0.1,
            action: SpriteAnimationAction::default(),
            mode: SpriteAnimationMode::default(),
        }
    }
    /// Set current Atlas Handle
    pub fn set_atlas_handle(&mut self, atlas_handle: Handle<TextureAtlas>) -> &mut Self {
        self.atlas_handle = atlas_handle;
        self
    }
    /// Set current Atlas Index
    pub fn set_atlas_index(&mut self, atlas_index: u32) -> &mut Self {
        self.atlas_index = atlas_index;
        self
    }
    /// Set current Frame Duration
    pub fn set_duration(&mut self, duration: f32) -> &mut Self {
        self.duration = duration;
        self
    }
    /// Set Sprite Animation Mode
    pub fn set_mode(&mut self, mode: SpriteAnimationMode) -> &mut Self {
        self.mode = mode;
        self
    }
    /// Control Sprite Animation Completed Action
    pub fn set_completed_action(&mut self, action: SpriteAnimationAction) -> &mut Self {
        self.action = action;
        self
    }
    /// Create new frame from current builder state.
    pub fn add_frame(&mut self) -> &mut Self {
        self.frames.push(SpriteAnimationFrame {
            atlas_handle: self.atlas_handle.clone(),
            atlas_index: self.atlas_index,
            duration: self.duration,
        });
        self
    }
    /// Get current count of frames in animation.
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }
    /// Create animation from current frames.
    pub fn finish(self) -> SpriteAnimation {
        SpriteAnimation {
            frames: self.frames,
            action: self.action,
            mode: self.mode,
        }
    }
    /// Begin building a new Sprite Animation
    pub fn quick_build(
        atlas_handle: Handle<TextureAtlas>,
        atlas_index_range: Range<u32>,
        duration: f32,
        action: SpriteAnimationAction,
        mode: SpriteAnimationMode,
    ) -> SpriteAnimation {
        SpriteAnimation {
            frames: atlas_index_range
                .map(|atlas_index| SpriteAnimationFrame {
                    atlas_handle: atlas_handle.clone(),
                    atlas_index,
                    duration,
                })
                .collect(),
            mode,
            action,
        }
    }
}

pub fn sprite_animation_system(
    time: Res<Time>,
    catalog: Res<SpriteCatalog>,
    mut query: Query<(
        &mut SpriteAnimator,
        &mut Handle<TextureAtlas>,
        &mut TextureAtlasSprite,
    )>,
) {
    for (mut animator, mut atlas, mut sprite) in &mut query.iter_mut() {
        animator.sprite_timer.tick(time.delta());
        if let Some(collection_idx) = animator.current_collection {
            if let Some(collection) = catalog.collections.get(collection_idx) {
                if let Some(animation_idx) = animator.current_animation {
                    if let Some(animation) = collection.animations.get(animation_idx) {
                        // Identify next_frame
                        let next_frame = if let Some(frame_idx) = animator.current_frame {
                            // Advance to next Animation Frame based on mode.
                            match animation.mode {
                                SpriteAnimationMode::Forward => {
                                    if frame_idx + 1 < animation.frames.len() {
                                        frame_idx + 1
                                    } else {
                                        match animation.action {
                                            SpriteAnimationAction::DisableAnimation => {
                                                animator.current_animation = None;
                                                animator.current_frame = None;
                                            }
                                            SpriteAnimationAction::RepeatAnimation => {
                                                animator.current_frame = 0;
                                            }
                                            SpriteAnimationAction::RepeatAnimation => {
                                                animator.current_frame = 0;
                                            }
                                        }
                                    }
                                }
                                SpriteAnimationMode::Reverse => {
                                    if frame_idx > 0 {
                                        frame_idx - 1
                                    } else if animation.repeat {
                                        animation.frames.len() - 1
                                    } else {
                                        frame_idx
                                    }
                                }
                                SpriteAnimationMode::PingPong => {
                                    let next_frame = frame_idx + 1;
                                    if next_frame < animation.frames.len() {
                                        // Going up
                                        next_frame
                                    } else if next_frame < animation.frames.len() * 2 - 1 {
                                        // Going Down
                                        animation.frames.len() * 2 - next_frame - 2
                                    } else {
                                        // Back at 0
                                        if animation.repeat {
                                            0
                                        } else {
                                            frame_idx
                                        }
                                    }
                                }
                            }
                        } else {
                            // Initialize Animation as no frame exists
                            match animation.mode {
                                SpriteAnimationMode::Forward => 0,
                                SpriteAnimationMode::Reverse => animation.frames.len() - 1,
                                SpriteAnimationMode::PingPong => 0,
                            }
                        };
                        if let Some(frame) = animation.frames.get(next_frame) {
                            // Set the actual Sprite Information
                            let duration = Duration::from_secs_f32(frame.duration);
                            animator.sprite_timer.set_duration(duration);
                            animator.current_frame = Some(next_frame);
                            *atlas = frame.atlas_handle.clone();
                            sprite.index = frame.atlas_index;
                        } else {
                            event!(
                                Level::DEBUG,
                                "Frame {:?} not found for animation {:?} in collection {:?}.",
                                animator.current_frame,
                                animator.current_animation,
                                animator.current_collection,
                            );
                        };
                    } else {
                        event!(
                            Level::DEBUG,
                            "Animation {:?} not found for Collection {:?}.",
                            animation_idx,
                            collection_idx
                        );
                    }
                } else {
                    // No animation Selected. Do Nothing
                    continue;
                }
            } else {
                // Missing Collection
                event!(Level::DEBUG, "Collection {:?} not found.", collection_idx);
            }
        } else {
            // No collection Selected. Do Nothing
            continue;
        }
    }
}

/// Advance the Sprite Animator and return if this method should be called a second time
fn advance_animator(&mut animator: SpriteAnimator, &animation: SpriteAnimation) -> bool {
    if let Some(frame_idx) = animator.current_frame {
        // Animation has already started and we should advance to the next frame based on mode.
        match animation.mode {
            SpriteAnimationMode::Forward => {
                if frame_idx + 1 < animation.frames.len() {
                    frame_idx + 1;
                    return false;
                } else {
                    // Animation has completed, change Animator state based on action
                    match animation.action {
                        SpriteAnimationAction::DisableAnimation => {
                            animator.current_animation = None;
                            animator.current_frame = None;
                            return true;
                        }
                        SpriteAnimationAction::RepeatAnimation => {
                            animator.current_frame = None;
                            return true;
                        }
                        SpriteAnimationAction::RepeatAnimation => {
                            animator.current_frame = 0;
                        }
                    }
                }
            }
            SpriteAnimationMode::Reverse => {
                if frame_idx > 0 {
                    frame_idx - 1
                } else if animation.repeat {
                    animation.frames.len() - 1
                } else {
                    frame_idx
                }
            }
            SpriteAnimationMode::PingPong => {
                let next_frame = frame_idx + 1;
                if next_frame < animation.frames.len() {
                    // Going up
                    next_frame
                } else if next_frame < animation.frames.len() * 2 - 1 {
                    // Going Down
                    animation.frames.len() * 2 - next_frame - 2
                } else {
                    // Back at 0
                    if animation.repeat {
                        0
                    } else {
                        frame_idx
                    }
                }
            }
        }
    } else {
        // Initialize Animation as no frame exists
        match animation.mode {
            SpriteAnimationMode::Forward => 0,
            SpriteAnimationMode::Reverse => animation.frames.len() - 1,
            SpriteAnimationMode::PingPong => 0,
        }
    };
}
