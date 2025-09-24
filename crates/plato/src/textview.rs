use std::path::Path;
use std::sync::mpsc::Sender;
use std::collections::VecDeque;
use std::fs;

use plato_core::anyhow::Error;
use plato_core::geom::Rectangle;
use plato_core::framebuffer::{Framebuffer, UpdateMode};
use plato_core::context::Context;
use plato_core::font::Fonts;
use plato_core::view::{View, Event, Hub, Bus, RenderQueue, RenderData, ID_FEEDER, Id};

pub struct TextView {
    id: Id,
    rect: Rectangle,
    text: String,
    children: Vec<Box<dyn View>>, // most views keep this
}

impl TextView {
    pub fn new(
        rect: Rectangle,
        path: &Path,
        _hub: &Sender<Event>,
        _rq: &mut RenderQueue,
        _ctx: &mut Context,
    ) -> Result<Self, Error> {
        let text = fs::read_to_string(path).unwrap_or_else(|_| "".into());

        Ok(Self {
            id: ID_FEEDER.next(),
            rect,
            text,
            children: Vec::new(),
        })
    }
}

impl View for TextView {
    fn handle_event(
        &mut self,
        evt: &Event,
        _hub: &Hub,
        _bus: &mut Bus,
        rq: &mut RenderQueue,
        _ctx: &mut Context,
    ) -> bool {
        match evt {
            Event::Back => {
                // If Back is pressed, close the view
                return false;
            }
            Event::Keyboard(k) => {
                // TODO: handle keyboard events here
                println!("Keyboard event: {:?}", k);
            }
            _ => {}
        }
        rq.add(RenderData::new(self.id, self.rect, UpdateMode::Gui));
        true
    }

    fn render(&self, fb: &mut dyn Framebuffer, _rect: Rectangle, fonts: &mut Fonts) {
        // TODO: actually draw text on screen
        // For now, nothing is rendered
        let _ = (fb, fonts);
    }

    fn rect(&self) -> &Rectangle {
        &self.rect
    }

    fn rect_mut(&mut self) -> &mut Rectangle {
        &mut self.rect
    }

    fn children(&self) -> &Vec<Box<dyn View>> {
        &self.children
    }

    fn children_mut(&mut self) -> &mut Vec<Box<dyn View>> {
        &mut self.children
    }

    fn id(&self) -> Id {
        self.id
    }
}
