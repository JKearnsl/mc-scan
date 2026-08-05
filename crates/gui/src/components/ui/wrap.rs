//! A flex-wrap row: children keep their intrinsic width and wrap to a new line
//! when the next one would overflow. Height shrinks to content, width fills the
//! parent. A fixed N-column grid can't do this — it forces one slot per cell.

use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::{Tree, Widget};
use iced::advanced::{Clipboard, Shell, mouse};
use iced::{Element, Event, Length, Point, Rectangle, Size, Theme};

pub struct Wrap<'a, Message> {
    children: Vec<Element<'a, Message>>,
    spacing: f32,
    line_spacing: f32,
}

/// A flex-wrap row of `children`. Set the gaps with [`Wrap::spacing`].
pub fn wrap<'a, Message>(children: Vec<Element<'a, Message>>) -> Wrap<'a, Message> {
    Wrap {
        children,
        spacing: 0.0,
        line_spacing: 0.0,
    }
}

impl<Message> Wrap<'_, Message> {
    /// Sets both the horizontal gap between items and the vertical gap between
    /// lines.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self.line_spacing = spacing;
        self
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Wrap<'_, Message> {
    fn children(&self) -> Vec<Tree> {
        self.children.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.children);
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Shrink)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let max_width = limits.max().width;
        let child_limits = layout::Limits::new(Size::ZERO, Size::new(max_width, f32::INFINITY));

        let mut nodes = Vec::with_capacity(self.children.len());
        let mut x = 0.0f32;
        let mut y = 0.0f32;
        let mut line_height = 0.0f32;

        for (child, child_tree) in self.children.iter_mut().zip(&mut tree.children) {
            let mut node = child
                .as_widget_mut()
                .layout(child_tree, renderer, &child_limits);
            let size = node.size();

            // Wrap before placing when this child would overflow the current line
            // (but never wrap an empty line — a lone oversized child stays put).
            if x > 0.0 && x + size.width > max_width {
                x = 0.0;
                y += line_height + self.line_spacing;
                line_height = 0.0;
            }

            node.move_to_mut(Point::new(x, y));
            line_height = line_height.max(size.height);
            x += size.width + self.spacing;
            nodes.push(node);
        }

        let total_height = y + line_height;
        layout::Node::with_children(Size::new(max_width, total_height), nodes)
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
            .children
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
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        for ((child, child_tree), child_layout) in self
            .children
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
        {
            child.as_widget().draw(
                child_tree,
                renderer,
                theme,
                style,
                child_layout,
                cursor,
                viewport,
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
            .children
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

impl<'a, Message: 'a> From<Wrap<'a, Message>> for Element<'a, Message> {
    fn from(w: Wrap<'a, Message>) -> Self {
        Element::new(w)
    }
}
