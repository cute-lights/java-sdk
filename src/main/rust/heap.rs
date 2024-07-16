use crate::heap;

heap!(light, Box<dyn cute_lights::Light>);
heap!(frame, crate::frame::Frame);