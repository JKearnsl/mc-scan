use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::{Operation, Tree, tree};
use iced::advanced::{Clipboard, Shell, Widget, mouse, overlay, renderer};
use iced::{Element, Event, Length, Point, Rectangle, Size, Theme, Vector};

const GAP: f32 = 6.0;
const MARGIN: f32 = 8.0;

pub struct Popover<'a, Message> {
    open: bool,
    on_dismiss: Message,
    // [0] = trigger (laid out inline), [1] = panel (shown only as an overlay).
    children: Vec<Element<'a, Message>>,
}

pub fn popover<'a, Message>(
    trigger: impl Into<Element<'a, Message>>,
    panel: impl Into<Element<'a, Message>>,
    open: bool,
    on_dismiss: Message,
) -> Popover<'a, Message> {
    Popover {
        open,
        on_dismiss,
        children: vec![trigger.into(), panel.into()],
    }
}

impl<Message: Clone> Widget<Message, Theme, iced::Renderer> for Popover<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::stateless()
    }

    fn children(&self) -> Vec<Tree> {
        self.children.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.children);
    }

    fn size(&self) -> Size<Length> {
        self.children[0].as_widget().size()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let child =
            self.children[0]
                .as_widget_mut()
                .layout(&mut tree.children[0], renderer, limits);
        layout::Node::with_children(child.size(), vec![child])
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
        self.children[0].as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout.children().next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
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
        self.children[0].as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout.children().next().unwrap(),
            cursor,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        self.children[0].as_widget().mouse_interaction(
            &tree.children[0],
            layout.children().next().unwrap(),
            cursor,
            viewport,
            renderer,
        )
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        self.children[0].as_widget_mut().operate(
            &mut tree.children[0],
            layout.children().next().unwrap(),
            renderer,
            operation,
        );
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        _renderer: &iced::Renderer,
        _viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, iced::Renderer>> {
        if !self.open {
            return None;
        }
        let anchor = layout.children().next().unwrap().bounds() + translation;
        let (_, panel) = self.children.split_at_mut(1);
        let (_, panel_tree) = tree.children.split_at_mut(1);
        Some(overlay::Element::new(Box::new(PanelOverlay {
            panel: &mut panel[0],
            tree: &mut panel_tree[0],
            anchor,
            on_dismiss: self.on_dismiss.clone(),
        })))
    }
}

struct PanelOverlay<'a, 'b, Message> {
    panel: &'b mut Element<'a, Message>,
    tree: &'b mut Tree,
    anchor: Rectangle,
    on_dismiss: Message,
}

impl<Message: Clone> overlay::Overlay<Message, Theme, iced::Renderer>
    for PanelOverlay<'_, '_, Message>
{
    fn layout(&mut self, renderer: &iced::Renderer, bounds: Size) -> layout::Node {
        let limits = layout::Limits::new(Size::ZERO, bounds);
        let mut node = self
            .panel
            .as_widget_mut()
            .layout(self.tree, renderer, &limits);
        let size = node.size();

        // Right-align to the trigger; flip above when there's no room below.
        let mut x = self.anchor.x + self.anchor.width - size.width;
        x = x.clamp(MARGIN, (bounds.width - size.width - MARGIN).max(MARGIN));

        let below = self.anchor.y + self.anchor.height + GAP;
        let y = if below + size.height <= bounds.height - MARGIN {
            below
        } else {
            (self.anchor.y - GAP - size.height).max(MARGIN)
        };

        node.move_to_mut(Point::new(x, y));
        node
    }

    fn draw(
        &self,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        self.panel.as_widget().draw(
            self.tree,
            renderer,
            theme,
            style,
            layout,
            cursor,
            &layout.bounds(),
        );
    }

    fn update(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) {
        let over_panel = cursor.is_over(layout.bounds());

        // Swallow the outside press so it can't re-toggle the trigger beneath.
        if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event
            && !over_panel
        {
            shell.publish(self.on_dismiss.clone());
            shell.capture_event();
            return;
        }

        self.panel.as_widget_mut().update(
            self.tree,
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            &layout.bounds(),
        );

        // Consume mouse events over the panel so they don't fall through to the list.
        if over_panel && matches!(event, Event::Mouse(_)) {
            shell.capture_event();
        }
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        let inner = self.panel.as_widget().mouse_interaction(
            self.tree,
            layout,
            cursor,
            &layout.bounds(),
            renderer,
        );
        // Claim the cursor over inert panel areas too, so the list stops hovering through it.
        if inner == mouse::Interaction::None && cursor.is_over(layout.bounds()) {
            mouse::Interaction::Idle
        } else {
            inner
        }
    }

    fn operate(
        &mut self,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        self.panel
            .as_widget_mut()
            .operate(self.tree, layout, renderer, operation);
    }
}

impl<'a, Message: Clone + 'a> From<Popover<'a, Message>> for Element<'a, Message> {
    fn from(popover: Popover<'a, Message>) -> Self {
        Element::new(popover)
    }
}
