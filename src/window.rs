extern crate window;

use std::rc::Rc;
use std::cell::RefCell;
use glium::backend::{ Backend, Context, Facade };
use glium::{ GliumCreationError, Frame, SwapBuffersError };
use self::window::{ OpenGLWindow, ProcAddress };

#[derive(Clone)]
struct Wrapper<W>(Rc<RefCell<W>>);

/// A window struct for glium.
#[derive(Clone)]
pub struct GliumWindow(Rc<Context>);

impl GliumWindow {
    /// Creates new GliumWindow.
    pub fn new<W>(window: &Rc<RefCell<W>>) -> Result<Self, GliumCreationError<()>>
        where W: OpenGLWindow + 'static
    {
        unsafe {
            Context::new(Wrapper(window.clone()), true, Default::default())
        }.map(GliumWindow)
    }

    /// Returns new frame.
    pub fn draw(&self) -> Frame {
        Frame::new(self.0.clone(), self.0.get_framebuffer_dimensions())
    }
}

impl Facade for GliumWindow {
    fn get_context(&self) -> &Rc<Context> {
        &self.0
    }
}

unsafe impl<W> Backend for Wrapper<W> where W: OpenGLWindow {
    fn swap_buffers(&self) -> Result<(), SwapBuffersError> {
        self.0.borrow_mut().swap_buffers();
        Ok(())
    }

    unsafe fn get_proc_address(&self, proc_name: &str) -> ProcAddress {
        self.0.borrow_mut().get_proc_address(proc_name)
    }

    fn get_framebuffer_dimensions(&self) -> (u32, u32) {
        let size = self.0.borrow().size();
        (size.width, size.height)
    }

    fn is_current(&self) -> bool {
        self.0.borrow().is_current()
    }

    unsafe fn make_current(&self) {
        self.0.borrow_mut().make_current()
    }
}
