extern crate piston;

use std::rc::Rc;
use std::cell::RefCell;
use std::os::raw::c_void;
use graphics::{ self, Viewport };
use glium::backend::{ Backend, Context, Facade };
use glium::{ GliumCreationError, Frame, SwapBuffersError };
use self::piston::event_loop::{ EventLoop, WindowEvents };
use self::piston::window::{ BuildFromWindowSettings, OpenGLWindow, Window, WindowSettings };
use self::piston::input::{ Event, GenericEvent };

use Glium2d;
use GliumGraphics;

#[derive(Clone)]
struct Wrapper<W>(Rc<RefCell<W>>);

/// A window struct for glium.
pub struct GliumWindow<W> {
    /// Window.
    pub window: Rc<RefCell<W>>,
    /// Glium context.
    pub context: Rc<Context>,
    /// Event loop state.
    pub events: WindowEvents
}

impl<W> Clone for GliumWindow<W> {
    fn clone(&self) -> GliumWindow<W> {
        GliumWindow {
            window: self.window.clone(),
            context: self.context.clone(),
            events: self.events.clone()
        }
    }
}

impl<W> BuildFromWindowSettings for GliumWindow<W>
    where W: 'static + Window + OpenGLWindow + BuildFromWindowSettings,
          W::Event: GenericEvent
{
    fn build_from_window_settings(mut settings: WindowSettings)
    -> Result<GliumWindow<W>, String> {
        // Turn on sRGB.
        settings = settings.srgb(true);
        GliumWindow::new(&Rc::new(RefCell::new(try!(settings.build()))))
            .map_err(|err| match err {
                GliumCreationError::BackendCreationError(..) =>
                    "Error while creating the backend",
                GliumCreationError::IncompatibleOpenGl(..) =>
                    "The OpenGL implementation is too old to work with glium",
            }.into())
    }
}

impl<W> GliumWindow<W>
    where W: OpenGLWindow + 'static
{
    /// Creates new GliumWindow.
    pub fn new(window: &Rc<RefCell<W>>) -> Result<Self, GliumCreationError<()>> {
        unsafe {
            Context::new(Wrapper(window.clone()), true, Default::default())
        }.map(|context| GliumWindow {
            window: window.clone(),
            context: context,
            events: WindowEvents::new().swap_buffers(false)
        })
    }

    /// Returns new frame.
    pub fn draw(&self) -> Frame {
        Frame::new(self.context.clone(), self.context.get_framebuffer_dimensions())
    }

    /// Renders 2D graphics.
    pub fn draw_2d<F>(&mut self, target: &mut Frame, g2d: &mut Glium2d, viewport: Viewport, f: F) where
        F: FnOnce(graphics::Context, &mut GliumGraphics<Frame>)
    {
        use graphics::Context;

        let ref mut g = GliumGraphics::new(g2d, target);
        let c = Context::new_viewport(viewport);
        f(c, g);
    }

    /// Returns next event.
    pub fn next(&mut self) -> Option<Event<<W as Window>::Event>> {
        self.events.next(&mut *self.window.borrow_mut())
    }
}

impl<W> Facade for GliumWindow<W> {
    fn get_context(&self) -> &Rc<Context> {
        &self.context
    }
}

unsafe impl<W> Backend for Wrapper<W> where W: OpenGLWindow {
    fn swap_buffers(&self) -> Result<(), SwapBuffersError> {
        self.0.borrow_mut().swap_buffers();
        Ok(())
    }

    unsafe fn get_proc_address(&self, proc_name: &str) -> *const c_void {
        self.0.borrow_mut().get_proc_address(proc_name) as *const c_void
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
