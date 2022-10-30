use super::*;

pub struct Vec2Slider<'a> {
    pub position: &'a mut AABB<f32>,
    pub sense: &'a mut geng::ui::Sense,
    pub animation: &'a mut f32,
    pub change: RefCell<&'a mut Option<Vec2<f32>>>,
    pub value: Vec2<f32>,
}

impl<'a> Vec2Slider<'a> {
    pub fn new(cx: &'a geng::ui::Controller, value: Vec2<f32>) -> Self {
        Self {
            position: cx.get_state_with(|| AABB::point(Vec2::ZERO)),
            sense: cx.get_state(),
            animation: cx.get_state(),
            change: RefCell::new(cx.get_state()),
            value,
        }
    }

    pub fn get_change(&self) -> Option<Vec2<f32>> {
        self.change.borrow_mut().take()
    }
}

impl geng::ui::Widget for Vec2Slider<'_> {
    fn calc_constraints(
        &mut self,
        children: &geng::ui::ConstraintsContext,
    ) -> geng::ui::Constraints {
        geng::ui::Constraints {
            min_size: vec2(100.0, 100.0),
            flex: vec2(0.0, 0.0),
        }
    }
    // If using Sense helper this method must be added
    fn sense(&mut self) -> Option<&mut geng::ui::Sense> {
        Some(self.sense)
    }
    fn update(&mut self, delta_time: f64) {
        let delta_time = delta_time as f32 * 10.0;
        if self.sense.is_hovered() || self.sense.is_captured() {
            *self.animation += delta_time;
        } else {
            *self.animation -= delta_time;
        }
        *self.animation = self.animation.clamp(0.0, 1.0);
    }
    fn draw(&mut self, cx: &mut geng::ui::DrawContext) {
        *self.position = cx.position.map(|x| x as f32); // Update hidden state to remember our widget's position

        cx.geng.draw_2d(
            cx.framebuffer,
            &geng::PixelPerfectCamera,
            &draw_2d::Ellipse {
                transform: Mat3::translate(self.position.center())
                    * Mat3::scale(self.position.size() / 2.0),
                cut: 0.0,
                color: cx.theme.usable_color,
            },
        );
        cx.geng.draw_2d(
            cx.framebuffer,
            &geng::PixelPerfectCamera,
            &draw_2d::Ellipse::circle(
                self.position.center() + self.position.size() * self.value / 2.0,
                5.0 + *self.animation * 5.0,
                cx.theme.hover_color,
            ),
        );
    }
    fn handle_event(&mut self, event: &geng::Event) {
        // Use helper to determine if we should process interactions
        if self.sense.is_captured() {
            if let geng::Event::MouseDown { position, .. }
            | geng::Event::MouseMove { position, .. } = &event
            {
                let new_value = ((position.map(|x| x as f32) - self.position.center())
                    / (self.position.size() / 2.0))
                    .clamp_len(..=1.0);
                **self.change.borrow_mut() = Some(new_value);
            }
        }
    }
}
