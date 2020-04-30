extern crate glutin_window;
extern crate piston;

use self::glutin_window::GlutinWindow;
use self::piston::event_loop::{EventLoop, EventSettings, Events};
use self::piston::input::Event;
use self::piston::window::{
    AdvancedWindow, BuildFromWindowSettings, OpenGLWindow, Position, Size, Window, WindowSettings,
};
use glium::backend::{Backend, Context, Facade};
use glium::{Frame, IncompatibleOpenGl, SwapBuffersError};
use std::cell::RefCell;
use std::error::Error;
use std::ops::Deref;
use std::os::raw::c_void;
use std::rc::Rc;
use std::time::Duration;

#[derive(Clone)]
struct Wrapper<W>(Rc<RefCell<W>>);

/// A window struct for glium.
pub struct GliumWindow<W = GlutinWindow> {
    /// Window.
    pub window: Rc<RefCell<W>>,
    /// Glium context.
    pub context: Rc<Context>,
    /// Event loop state.
    pub events: Events,
}

impl<W> Deref for GliumWindow<W> {
    type Target = Context;

    fn deref(&self) -> &Context {
        &self.context
    }
}

impl<W> Clone for GliumWindow<W> {
    fn clone(&self) -> GliumWindow<W> {
        GliumWindow {
            window: self.window.clone(),
            context: self.context.clone(),
            events: self.events.clone(),
        }
    }
}

impl<W> BuildFromWindowSettings for GliumWindow<W>
where
    W: 'static + Window + OpenGLWindow + BuildFromWindowSettings,
{
    fn build_from_window_settings(settings: &WindowSettings) -> Result<GliumWindow<W>, Box<dyn Error>> {
        let window: W = settings.clone().build()?;
        GliumWindow::new(&Rc::new(RefCell::new(window))).map_err(|err| err.into())
    }
}

impl<W> GliumWindow<W>
where
    W: OpenGLWindow + 'static,
{
    /// Creates new GliumWindow.
    pub fn new(window: &Rc<RefCell<W>>) -> Result<Self, IncompatibleOpenGl> {
        unsafe {
            let check_current = cfg!(feature = "check_current_window");
            Context::new(Wrapper(window.clone()), check_current, Default::default())
        }
        .map(|context| GliumWindow {
            window: window.clone(),
            context: context,
            events: Events::new(EventSettings::new()).swap_buffers(false),
        })
    }

    /// Returns new frame.
    pub fn draw(&self) -> Frame {
        Frame::new(
            self.context.clone(),
            self.context.get_framebuffer_dimensions(),
        )
    }

    /// Returns next event.
    pub fn next(&mut self) -> Option<Event> {
        self.events.next(&mut *self.window.borrow_mut())
    }
}

impl<W> Facade for GliumWindow<W> {
    fn get_context(&self) -> &Rc<Context> {
        &self.context
    }
}

unsafe impl<W> Backend for Wrapper<W>
where
    W: OpenGLWindow,
{
    fn swap_buffers(&self) -> Result<(), SwapBuffersError> {
        self.0.borrow_mut().swap_buffers();
        Ok(())
    }

    unsafe fn get_proc_address(&self, proc_name: &str) -> *const c_void {
        self.0.borrow_mut().get_proc_address(proc_name) as *const c_void
    }

    fn get_framebuffer_dimensions(&self) -> (u32, u32) {
        let size = self.0.borrow().draw_size();
        (size.width as u32, size.height as u32)
    }

    fn is_current(&self) -> bool {
        self.0.borrow().is_current()
    }

    unsafe fn make_current(&self) {
        self.0.borrow_mut().make_current()
    }
}

impl<W> Window for GliumWindow<W>
where
    W: Window,
{
    fn should_close(&self) -> bool {
        self.window.borrow().should_close()
    }
    fn set_should_close(&mut self, value: bool) {
        self.window.borrow_mut().set_should_close(value)
    }
    fn size(&self) -> Size {
        self.window.borrow().size()
    }
    fn draw_size(&self) -> Size {
        self.window.borrow().draw_size()
    }
    fn swap_buffers(&mut self) {
        self.window.borrow_mut().swap_buffers()
    }
    fn poll_event(&mut self) -> Option<Event> {
        Window::poll_event(&mut *self.window.borrow_mut())
    }
    fn wait_event(&mut self) -> Event {
        Window::wait_event(&mut *self.window.borrow_mut())
    }
    fn wait_event_timeout(&mut self, duration: Duration) -> Option<Event> {
        let mut window = self.window.borrow_mut();
        Window::wait_event_timeout(&mut *window, duration)
    }
}

impl<W> AdvancedWindow for GliumWindow<W>
where
    W: AdvancedWindow,
{
    fn get_title(&self) -> String {
        self.window.borrow().get_title()
    }
    fn set_title(&mut self, title: String) {
        self.window.borrow_mut().set_title(title)
    }
    fn get_automatic_close(&self) -> bool {
        self.window.borrow().get_automatic_close()
    }
    fn set_automatic_close(&mut self, value: bool) {
        self.window.borrow_mut().set_automatic_close(value);
    }
    fn get_exit_on_esc(&self) -> bool {
        self.window.borrow().get_exit_on_esc()
    }
    fn set_exit_on_esc(&mut self, value: bool) {
        self.window.borrow_mut().set_exit_on_esc(value)
    }
    fn set_capture_cursor(&mut self, value: bool) {
        self.window.borrow_mut().set_capture_cursor(value)
    }
    fn show(&mut self) {
        self.window.borrow_mut().show()
    }
    fn hide(&mut self) {
        self.window.borrow_mut().show()
    }
    fn get_position(&self) -> Option<Position> {
        self.window.borrow().get_position()
    }
    fn set_position<P: Into<Position>>(&mut self, pos: P) {
        self.window.borrow_mut().set_position(pos);
    }
    fn set_size<S: Into<Size>>(&mut self, size: S) {
        self.window.borrow_mut().set_size(size)
    }
}

impl<W> EventLoop for GliumWindow<W> {
    fn get_event_settings(&self) -> EventSettings {
        self.events.get_event_settings()
    }

    fn set_event_settings(&mut self, settings: EventSettings) {
        self.events.set_event_settings(settings);
    }
}
