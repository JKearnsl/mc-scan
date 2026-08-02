//! A virtualized vertical list of cards of a fixed height.
//!
//! Unlike `column` inside `scrollable`, it constructs and lays out **only**
//! cards visible in the window, so the frame cost does not depend on the total number
//! of results. It is itself a scroll container: it maintains the offset in the state,
//! handles the wheel and the slider dragging, and draws its own scrollbar.
//!
//! The technique of constructing content in `layout` (rather than `view`) is borrowed from
//! `iced::widget::responsive`: `diff` is deferred, and reconciliation of the state tree of
//! children is done via `tree.diff_children` after the
//! visible range is known.

use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer::{self, Quad, Renderer as _};
use iced::advanced::widget::{Tree, tree};
use iced::advanced::{Clipboard, Shell, Widget, mouse};
use iced::{Background, Border, Element, Event, Length, Point, Rectangle, Size, Theme};

const SCROLLBAR_WIDTH: f32 = 4.0;
const SCROLLBAR_MARGIN: f32 = 6.0;
const MIN_THUMB: f32 = 24.0;
const LINE_SCROLL: f32 = 60.0;

#[derive(Default)]
struct State {
    offset: f32,
    grab: Option<f32>,
}

pub struct VirtualList<'a, Message> {
    count: usize,
    row_height: f32,
    spacing: f32,
    padding: iced::Padding,
    build: Box<dyn Fn(usize) -> Element<'a, Message> + 'a>,
    content: Vec<Element<'a, Message>>,
}

impl<'a, Message> VirtualList<'a, Message> {
    pub fn new(
        count: usize,
        row_height: f32,
        spacing: f32,
        padding: iced::Padding,
        build: impl Fn(usize) -> Element<'a, Message> + 'a,
    ) -> Self {
        Self {
            count,
            row_height,
            spacing,
            padding,
            build: Box::new(build),
            content: Vec::new(),
        }
    }

    fn stride(&self) -> f32 {
        self.row_height + self.spacing
    }

    fn content_height(&self) -> f32 {
        if self.count == 0 {
            0.0
        } else {
            self.padding.top
                + self.padding.bottom
                + self.count as f32 * self.row_height
                + (self.count as f32 - 1.0) * self.spacing
        }
    }

    fn scrollbar(&self, bounds: Rectangle, offset: f32) -> Option<Scrollbar> {
        let content_h = self.content_height();
        let max_scroll = (content_h - bounds.height).max(0.0);
        if max_scroll <= 0.0 {
            return None;
        }
        let track_y = bounds.y + SCROLLBAR_MARGIN;
        let track_h = (bounds.height - 2.0 * SCROLLBAR_MARGIN).max(0.0);
        let thumb_h = (track_h * bounds.height / content_h).clamp(MIN_THUMB.min(track_h), track_h);
        let travel = (track_h - thumb_h).max(0.0);
        let thumb_y = track_y + travel * (offset / max_scroll);
        Some(Scrollbar {
            x: bounds.x + bounds.width - SCROLLBAR_MARGIN - SCROLLBAR_WIDTH,
            track_y,
            thumb_y,
            thumb_h,
            travel,
            max_scroll,
        })
    }
}

struct Scrollbar {
    x: f32,
    track_y: f32,
    thumb_y: f32,
    thumb_h: f32,
    travel: f32,
    max_scroll: f32,
}

impl Scrollbar {
    fn thumb_bounds(&self) -> Rectangle {
        Rectangle {
            x: self.x,
            y: self.thumb_y,
            width: SCROLLBAR_WIDTH,
            height: self.thumb_h,
        }
    }
    fn offset_for(&self, thumb_top: f32) -> f32 {
        if self.travel <= 0.0 {
            0.0
        } else {
            ((thumb_top - self.track_y) / self.travel).clamp(0.0, 1.0) * self.max_scroll
        }
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for VirtualList<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fill)
    }

    fn diff(&self, _tree: &mut Tree) {}

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits.width(Length::Fill).height(Length::Fill);
        let size = limits.max();
        let (vw, vh) = (size.width, size.height);

        let stride = self.stride();
        let max_scroll = (self.content_height() - vh).max(0.0);

        let state = tree.state.downcast_mut::<State>();
        state.offset = state.offset.clamp(0.0, max_scroll);
        let offset = state.offset;

        let (first, last) = if self.count == 0 {
            (0, 0)
        } else {
            let f = ((offset - self.padding.top) / stride).floor();
            let first = if f < 0.0 {
                0
            } else {
                (f as usize).min(self.count)
            };
            let l = ((offset + vh - self.padding.top) / stride).ceil();
            let last = (l.max(0.0) as usize).clamp(first, self.count);
            (first, last)
        };

        let mut built = Vec::with_capacity(last - first);
        for i in first..last {
            built.push((self.build)(i));
        }
        self.content = built;
        tree.diff_children(&self.content);

        let inner_w = (vw - self.padding.left - self.padding.right).max(0.0);
        let child_limits = layout::Limits::new(
            Size::new(inner_w, self.row_height),
            Size::new(inner_w, self.row_height),
        );

        let mut nodes = Vec::with_capacity(self.content.len());
        for (k, elem) in self.content.iter_mut().enumerate() {
            let i = first + k;
            let mut node =
                elem.as_widget_mut()
                    .layout(&mut tree.children[k], renderer, &child_limits);
            let y = self.padding.top + i as f32 * stride - offset;
            node.move_to_mut(Point::new(self.padding.left, y));
            nodes.push(node);
        }

        layout::Node::with_children(Size::new(vw, vh), nodes)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        for ((child, child_tree), child_layout) in self
            .content
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
        {
            child.as_widget_mut().update(
                child_tree,
                event,
                child_layout,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );
        }

        if shell.is_event_captured() {
            return;
        }

        let bounds = layout.bounds();

        match event {
            Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                if cursor.position_over(bounds).is_none() {
                    return;
                }
                let dy = match *delta {
                    mouse::ScrollDelta::Lines { y, .. } => -y * LINE_SCROLL,
                    mouse::ScrollDelta::Pixels { y, .. } => -y,
                };
                let max_scroll = (self.content_height() - bounds.height).max(0.0);
                let state = tree.state.downcast_mut::<State>();
                let before = state.offset;
                state.offset = (state.offset + dy).clamp(0.0, max_scroll);
                if state.offset != before {
                    shell.capture_event();
                    shell.invalidate_layout();
                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                let Some(pos) = cursor.position() else { return };
                let offset = tree.state.downcast_ref::<State>().offset;
                let Some(sb) = self.scrollbar(bounds, offset) else {
                    return;
                };
                let grab = if sb.thumb_bounds().contains(pos) {
                    pos.y - sb.thumb_y
                } else if pos.x >= sb.x - SCROLLBAR_MARGIN && bounds.contains(pos) {
                    let new_offset = sb.offset_for(pos.y - sb.thumb_h / 2.0);
                    tree.state.downcast_mut::<State>().offset = new_offset;
                    shell.invalidate_layout();
                    shell.request_redraw();
                    sb.thumb_h / 2.0
                } else {
                    return;
                };
                tree.state.downcast_mut::<State>().grab = Some(grab);
                shell.capture_event();
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                let grab = tree.state.downcast_ref::<State>().grab;
                let Some(grab) = grab else { return };
                let Some(pos) = cursor.position() else { return };
                let offset = tree.state.downcast_ref::<State>().offset;
                if let Some(sb) = self.scrollbar(bounds, offset) {
                    let new_offset = sb.offset_for(pos.y - grab);
                    let state = tree.state.downcast_mut::<State>();
                    if state.offset != new_offset {
                        state.offset = new_offset;
                        shell.invalidate_layout();
                        shell.request_redraw();
                    }
                }
                shell.capture_event();
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                let state = tree.state.downcast_mut::<State>();
                if state.grab.take().is_some() {
                    shell.request_redraw();
                }
            }
            _ => {}
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();

        renderer.with_layer(bounds, |renderer| {
            for ((child, child_tree), child_layout) in self
                .content
                .iter()
                .zip(&tree.children)
                .zip(layout.children())
            {
                if child_layout.bounds().intersects(&bounds) {
                    child.as_widget().draw(
                        child_tree,
                        renderer,
                        theme,
                        style,
                        child_layout,
                        cursor,
                        &bounds,
                    );
                }
            }
        });

        let offset = tree.state.downcast_ref::<State>().offset;
        if let Some(sb) = self.scrollbar(bounds, offset) {
            let dark = crate::styles::is_dark(theme);
            let rail_color = if dark {
                crate::styles::c("#1A1F27")
            } else {
                crate::styles::c("#E1E5EA")
            };
            let thumb_color = if dark {
                crate::styles::c("#232A34")
            } else {
                crate::styles::c("#C8CDD5")
            };
            let track_h = (bounds.height - 2.0 * SCROLLBAR_MARGIN).max(0.0);
            renderer.fill_quad(
                Quad {
                    bounds: Rectangle {
                        x: sb.x,
                        y: sb.track_y,
                        width: SCROLLBAR_WIDTH,
                        height: track_h,
                    },
                    border: Border {
                        radius: 2.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Background::Color(rail_color),
            );
            renderer.fill_quad(
                Quad {
                    bounds: sb.thumb_bounds(),
                    border: Border {
                        radius: 2.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Background::Color(thumb_color),
            );
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        for ((child, child_tree), child_layout) in self
            .content
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
        {
            let interaction = child.as_widget().mouse_interaction(
                child_tree,
                child_layout,
                cursor,
                viewport,
                renderer,
            );
            if interaction != mouse::Interaction::None {
                return interaction;
            }
        }
        mouse::Interaction::None
    }
}

impl<'a, Message: 'a> From<VirtualList<'a, Message>> for Element<'a, Message> {
    fn from(list: VirtualList<'a, Message>) -> Self {
        Element::new(list)
    }
}
