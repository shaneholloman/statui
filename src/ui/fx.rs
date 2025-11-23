use ratatui::{Frame, layout::Rect, style::Color};
use tachyonfx::{Duration, EffectManager, Interpolation, Motion, fx};

pub struct FxManager {
    manager: EffectManager<()>,
}

impl FxManager {
    pub fn new() -> Self {
        FxManager {
            manager: EffectManager::default(),
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, elapsed: Duration) {
        self.manager
            .process_effects(elapsed, frame.buffer_mut(), area);
    }

    pub fn trigger_startup(&mut self) {
        let c = Color::from_u32(0x1d2021);
        let timer = (1000, Interpolation::Linear);
        let fx = fx::slide_in(Motion::UpToDown, 10, 0, c, timer);
        self.manager.add_effect(fx);
    }
}
