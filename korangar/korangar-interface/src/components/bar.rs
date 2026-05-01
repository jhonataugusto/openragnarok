use std::cell::Cell;
use std::marker::PhantomData;
use std::time::{Duration, Instant};

use rust_state::{Selector, State};

use crate::application::{Application, ShadowPadding};
use crate::element::Element;
use crate::element::store::{ElementStore, ElementStoreMut};
use crate::layout::alignment::{HorizontalAlignment, VerticalAlignment};
use crate::layout::{Resolvers, WindowLayout, with_single_resolver};

#[derive(Clone, Debug)]
pub struct SmoothFraction {
    current: Cell<f32>,
    start: Cell<f32>,
    target: Cell<f32>,
    started_at: Cell<Instant>,
    duration: Cell<Duration>,
    initialized: Cell<bool>,
}

impl Default for SmoothFraction {
    fn default() -> Self {
        let now = Instant::now();
        Self {
            current: Cell::new(0.0),
            start: Cell::new(0.0),
            target: Cell::new(0.0),
            started_at: Cell::new(now),
            duration: Cell::new(Duration::ZERO),
            initialized: Cell::new(false),
        }
    }
}

impl SmoothFraction {
    pub fn update(&self, target: f32, duration: Duration) -> f32 {
        self.update_at(target, duration, Instant::now())
    }

    pub fn update_at(&self, target: f32, duration: Duration, now: Instant) -> f32 {
        let target = target.clamp(0.0, 1.0);

        if !self.initialized.get() || duration.is_zero() {
            self.current.set(target);
            self.start.set(target);
            self.target.set(target);
            self.started_at.set(now);
            self.duration.set(Duration::ZERO);
            self.initialized.set(true);
            return self.current.get();
        }

        self.current.set(self.current_at(now));

        if (self.target.get() - target).abs() > f32::EPSILON {
            self.start.set(self.current.get());
            self.target.set(target);
            self.started_at.set(now);
            self.duration.set(duration);
        }

        self.current.set(self.current_at(now));
        self.current.get()
    }

    fn current_at(&self, now: Instant) -> f32 {
        let duration = self.duration.get();
        if duration.is_zero() {
            return self.target.get();
        }

        let progress = now.duration_since(self.started_at.get()).as_secs_f32() / duration.as_secs_f32();
        let progress = progress.clamp(0.0, 1.0);
        self.start.get() + (self.target.get() - self.start.get()) * progress
    }
}

pub struct Bar<Text, A, B, C, D, E, F, G, H, I, J, K, L, M> {
    text_marker: PhantomData<Text>,
    text: A,
    fraction: B,
    smooth_fraction: SmoothFraction,
    foreground_color: C,
    fill_color: D,
    background_color: E,
    highlight_color: F,
    height: G,
    corner_diameter: H,
    font_size: I,
    horizontal_alignment: J,
    vertical_alignment: K,
    overflow_behavior: L,
    lerp_duration_ms: M,
}

impl<Text, A, B, C, D, E, F, G, H, I, J, K, L, M> Bar<Text, A, B, C, D, E, F, G, H, I, J, K, L, M> {
    /// This function is supposed to be called from a component macro and not
    /// intended to be called manually.
    #[inline(always)]
    #[allow(clippy::too_many_arguments)]
    pub fn component_new(
        text: A,
        fraction: B,
        foreground_color: C,
        fill_color: D,
        background_color: E,
        highlight_color: F,
        height: G,
        corner_diameter: H,
        font_size: I,
        horizontal_alignment: J,
        vertical_alignment: K,
        overflow_behavior: L,
        lerp_duration_ms: M,
    ) -> Self {
        Self {
            text_marker: PhantomData,
            text,
            fraction,
            smooth_fraction: SmoothFraction::default(),
            foreground_color,
            fill_color,
            background_color,
            highlight_color,
            height,
            corner_diameter,
            font_size,
            horizontal_alignment,
            vertical_alignment,
            overflow_behavior,
            lerp_duration_ms,
        }
    }
}

impl<App, Text, A, B, C, D, E, F, G, H, I, J, K, L, M> Element<App> for Bar<Text, A, B, C, D, E, F, G, H, I, J, K, L, M>
where
    App: Application,
    Text: AsRef<str> + 'static,
    A: Selector<App, Text>,
    B: Selector<App, f32>,
    C: Selector<App, App::Color>,
    D: Selector<App, App::Color>,
    E: Selector<App, App::Color>,
    F: Selector<App, App::Color>,
    G: Selector<App, f32>,
    H: Selector<App, App::CornerDiameter>,
    I: Selector<App, App::FontSize>,
    J: Selector<App, HorizontalAlignment>,
    K: Selector<App, VerticalAlignment>,
    L: Selector<App, App::OverflowBehavior>,
    M: Selector<App, u32>,
{
    fn create_layout_info(&mut self, state: &State<App>, _: ElementStoreMut, resolvers: &mut dyn Resolvers<App>) -> Self::LayoutInfo {
        with_single_resolver(resolvers, |resolver| {
            let height = *state.get(&self.height);
            let text = state.get(&self.text).as_ref();
            let foreground_color = *state.get(&self.foreground_color);
            let highlight_color = *state.get(&self.highlight_color);
            let font_size = *state.get(&self.font_size);

            let (_size, font_size) = resolver.get_text_dimensions(
                text,
                foreground_color,
                highlight_color,
                font_size,
                *state.get(&self.horizontal_alignment),
                *state.get(&self.overflow_behavior),
            );

            let area = resolver.with_height(height);
            Self::LayoutInfo { area, font_size }
        })
    }

    fn lay_out<'a>(
        &'a self,
        state: &'a State<App>,
        _: ElementStore<'a>,
        layout_info: &'a Self::LayoutInfo,
        layout: &mut WindowLayout<'a, App>,
    ) {
        let area = layout_info.area;
        let corner_diameter = *state.get(&self.corner_diameter);

        layout.add_rectangle(
            area,
            corner_diameter,
            *state.get(&self.background_color),
            *state.get(&self.background_color),
            <App::ShadowPadding as ShadowPadding>::none(),
        );

        let target_fraction = *state.get(&self.fraction);
        let duration = Duration::from_millis(u64::from(*state.get(&self.lerp_duration_ms)));
        let fraction = self.smooth_fraction.update(target_fraction, duration);
        let fill_area = crate::layout::area::Area {
            width: area.width * fraction,
            ..area
        };

        if fill_area.width > 0.0 {
            layout.add_rectangle(
                fill_area,
                corner_diameter,
                *state.get(&self.fill_color),
                *state.get(&self.fill_color),
                <App::ShadowPadding as ShadowPadding>::none(),
            );
        }

        layout.add_text(
            area,
            state.get(&self.text).as_ref(),
            layout_info.font_size,
            *state.get(&self.foreground_color),
            *state.get(&self.highlight_color),
            *state.get(&self.horizontal_alignment),
            *state.get(&self.vertical_alignment),
            *state.get(&self.overflow_behavior),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smooth_fraction_initializes_to_first_target() {
        let now = Instant::now();
        let fraction = SmoothFraction::default();

        assert_eq!(fraction.update_at(0.75, Duration::from_millis(250), now), 0.75);
    }

    #[test]
    fn smooth_fraction_interpolates_to_changed_target() {
        let now = Instant::now();
        let fraction = SmoothFraction::default();
        fraction.update_at(1.0, Duration::from_millis(250), now);

        fraction.update_at(0.0, Duration::from_millis(250), now);
        let halfway = fraction.update_at(0.0, Duration::from_millis(250), now + Duration::from_millis(125));
        let done = fraction.update_at(0.0, Duration::from_millis(250), now + Duration::from_millis(250));

        assert!((halfway - 0.5).abs() < 0.01);
        assert_eq!(done, 0.0);
    }

    #[test]
    fn smooth_fraction_clamps_target() {
        let now = Instant::now();
        let fraction = SmoothFraction::default();

        assert_eq!(fraction.update_at(2.0, Duration::ZERO, now), 1.0);
        assert_eq!(fraction.update_at(-1.0, Duration::ZERO, now), 0.0);
    }
}
