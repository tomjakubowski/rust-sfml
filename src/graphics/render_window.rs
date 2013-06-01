/*!
* Window that can serve as a target for 2D drawing.
*
* RenderWindow is the main class of the Graphics module.
* It defines an OS window that can be painted using the other classes of the graphics module.
*
*/

use core::libc::{c_float, c_uint};
use rsfml::sfTypes::{sfBool};
use window::video_mode::*;
use window::context_settings::ContextSettings;
use window::event;
use window::keyboard;
use system::vector2;
use window::joystick;
use window::mouse;
use graphics::text::Text;
use graphics::drawable;
use graphics::color;
use graphics::sprite::Sprite;
use graphics::circle_shape::CircleShape;
use graphics::rectangle_shape::RectangleShape;
//use graphics::transform;
//use graphics::render_states;
use graphics::view;
use graphics::image;

#[doc(hidden)]
pub mod csfml {
    
    use core::libc::{c_uint, c_float, c_void, c_char};
    use system::vector2;
    use rsfml::sfTypes::{sfBool};
    use window::video_mode::*;
    use window::context_settings::ContextSettings;
    use graphics::text::csfml::sfText;
    use graphics::render_states;
    use graphics::color;
    use graphics::sprite::csfml::sfSprite;
    use graphics::circle_shape::csfml::sfCircleShape;
    use graphics::rectangle_shape::csfml::sfRectangleShape;
    use graphics::view::csfml::sfView;
    use graphics::image::csfml::sfImage;

    pub struct sfRenderWindow {
        This : *c_void
    }

    pub struct sfEvent {
        typeEvent : c_uint,
        p1 : c_uint,
        p2 : c_uint,
        p3 : c_float,
        p4 : c_uint,
        p5 : c_uint
    }

    pub extern "C" {
        fn sfRenderWindow_create(mode : csfml::sfVideoMode, title : *c_char, style : c_uint, settings : *ContextSettings) -> *sfRenderWindow;
        // fn sfRenderWindow_createUnicode(mode : sfVideoMode, title : *sfUint32, style : sfUint32, settings : *sfContextSettings) -> *sfRenderWindow;
        // fn sfRenderWindow_createFromHandle(handle : sfWindowHandle, settings : *sfContextSettings) -> *sfRenderWindow;
        fn sfRenderWindow_destroy(renderWindow : *sfRenderWindow) -> ();
        fn sfRenderWindow_close(renderWindow : *sfRenderWindow) -> ();
        fn sfRenderWindow_isOpen(renderWindow : *sfRenderWindow) -> sfBool;
        fn sfRenderWindow_getSettings(renderWindow : *sfRenderWindow) -> ContextSettings;
        fn sfRenderWindow_pollEvent(renderWindow : *sfRenderWindow, event : *sfEvent) -> sfBool;
        fn sfRenderWindow_waitEvent(renderWindow : *sfRenderWindow, event : *sfEvent) -> sfBool;
        fn sfRenderWindow_getPosition(renderWindow : *sfRenderWindow) -> vector2::csfml::sfVector2i;
        fn sfRenderWindow_setPosition(renderWindow : *sfRenderWindow, position : vector2::csfml::sfVector2i) -> ();
        fn sfRenderWindow_getSize(renderWindow : *sfRenderWindow) -> vector2::csfml::sfVector2u;
        fn sfRenderWindow_setSize(renderWindow : *sfRenderWindow, size : vector2::csfml::sfVector2u) -> ();
        fn sfRenderWindow_setTitle(renderWindow : *sfRenderWindow, title : *c_char) -> ();
        // fn sfRenderWindow_setUnicodeTitle(renderWindow : *sfRenderWindow, title : *sfUint32) -> ();
        // fn sfRenderWindow_setIcon(renderWindow : *sfRenderWindow, width : c_uint, height : c_uint, pixels : *sfUint8) -> ();
        fn sfRenderWindow_setVisible(renderWindow : *sfRenderWindow, visible : sfBool) -> ();
        fn sfRenderWindow_setMouseCursorVisible(renderWindow : *sfRenderWindow, show : sfBool) -> ();
        fn sfRenderWindow_setVerticalSyncEnabled(renderWindow : *sfRenderWindow, enabled : sfBool) -> ();
        fn sfRenderWindow_setKeyRepeatEnabled(renderWindow : *sfRenderWindow, enabled : sfBool) -> ();
        fn sfRenderWindow_setActive(renderWindow : *sfRenderWindow, active : sfBool) -> sfBool;
        fn sfRenderWindow_display(renderWindow : *sfRenderWindow) -> ();
        fn sfRenderWindow_setFramerateLimit(renderWindow : *sfRenderWindow, limit : c_uint) -> ();
        fn sfRenderWindow_setJoystickThreshold(renderWindow : *sfRenderWindow, treshold : c_float) -> ();
        // fn sfRenderWindow_getSystemHandle(renderWindow : *sfRenderWindow) -> sfWindowHandle;
        fn sfRenderWindow_clear(renderWindow : *sfRenderWindow, color : color::csfml::sfColor) -> ();
        fn sfRenderWindow_setView(renderWindow : *sfRenderWindow, view : *sfView) -> ();
        fn sfRenderWindow_getView(renderWindow : *sfRenderWindow) -> *sfView;
        fn sfRenderWindow_getDefaultView(renderWindow : *sfRenderWindow) -> *sfView;
        // fn sfRenderWindow_getViewport(renderWindow : *sfRenderWindow, view : *sfView) -> sfIntRect;
        fn sfRenderWindow_mapPixelToCoords(renderWindow : *sfRenderWindow, point : vector2::csfml::sfVector2i, view : *sfView) -> vector2::csfml::sfVector2f;
        fn sfRenderWindow_mapCoordsToPixel(renderWindow : *sfRenderWindow, point : vector2::csfml::sfVector2f, view : *sfView) -> vector2::csfml::sfVector2i;
        fn sfRenderWindow_drawSprite(renderWindow : *sfRenderWindow, object : *sfSprite, states : *render_states::csfml::sfRenderStates) -> ();
        fn sfRenderWindow_drawText(renderWindow : *sfRenderWindow, object : *sfText, states : *render_states::csfml::sfRenderStates) -> ();
        // fn sfRenderWindow_drawShape(renderWindow : *sfRenderWindow, object : *sfShape, states : *sfRenderStates) -> ();
        fn sfRenderWindow_drawCircleShape(renderWindow : *sfRenderWindow, object : *sfCircleShape, states : *render_states::csfml::sfRenderStates) -> ();
        // fn sfRenderWindow_drawConvexShape(renderWindow : *sfRenderWindow, object : *sfConvexShape, states : *sfRenderStates) -> ();
        fn sfRenderWindow_drawRectangleShape(renderWindow : *sfRenderWindow, object : *sfRectangleShape, states : *render_states::csfml::sfRenderStates) -> ();
        // fn sfRenderWindow_drawVertexArray(renderWindow : *sfRenderWindow, object : *sfVertexArray, states : *sfRenderStates) -> ();
        // fn sfRenderWindow_drawPrimitives(renderWindow : *sfRenderWindow, vertices : *sfVertex, vertexCount : c_uint, ttype : sfPrimitiveType, states : *sfRenderStates) -> ();  
        fn sfRenderWindow_pushGLStates(renderWindow : *sfRenderWindow) -> ();
        fn sfRenderWindow_popGLStates(renderWindow : *sfRenderWindow) -> ();
        fn sfRenderWindow_resetGLStates(renderWindow : *sfRenderWindow) -> ();
        fn sfRenderWindow_capture(renderWindow : *sfRenderWindow) -> *sfImage;
        fn sfMouse_getPositionRenderWindow(relativeTo : *sfRenderWindow) -> vector2::csfml::sfVector2i;
        fn sfMouse_setPositionRenderWindow(position : vector2::csfml::sfVector2i, relativeTo : *sfRenderWindow) -> ();
    }    
}

/// Enumeration of window creation styles
pub enum WindowStyle {
    pub sfNone = 0,
    pub sfTitlebar = 1,
    pub sfResize = 2,
    pub sfClose = 4,
    pub sfFullscreen = 8,
    pub sfDefaultStyle = 7
}

#[doc(hidden)]
pub struct RenderWindow {
    priv renderWindow : *csfml::sfRenderWindow,
    priv event : csfml::sfEvent
}

pub impl RenderWindow {
    pub fn new(mode : VideoMode, title : ~str, style : WindowStyle, settings : &ContextSettings) -> Option<RenderWindow> {
        let mut sfRenderWin: *csfml::sfRenderWindow = ptr::null();
        do str::as_c_str(title) |title_buf| {
            unsafe { sfRenderWin = csfml::sfRenderWindow_create(VideoMode::unwrap_videoMode(mode), title_buf, style as u32, settings); }
        };
        let sfEv : csfml::sfEvent = csfml::sfEvent {typeEvent : 0, p1 : 0, p2 : 0, p3 : 0 as c_float, p4 : 0, p5 : 0};//{0, 0, 0, 0 as float, 0, 0};
        if sfRenderWin == ptr::null() {
            None
        }
        else {
            Some (RenderWindow { renderWindow : sfRenderWin, event : sfEv})
        }
    }
    
    /**
    * Pop the event on top of event queue.
    *
    * Pop the event on top of event queue, if any, and return it, else return NoEvent.
    *
    */
    fn poll_event(&self) -> event::Event {
        let haveEvent : bool =  unsafe {
            match csfml::sfRenderWindow_pollEvent(self.renderWindow, &self.event) {
                0       => false,
                _       => true
            }
        };
        if haveEvent == false {
            return event::NoEvent;
        }
        self.get_wrapped_event()
    }
    
    /**
    * Wait for an event and return it
    *
    * wait_event is blocking, it wait until a new event arrive.
    *
    */
    fn wait_event(&self) -> event::Event {
        let haveEvent : bool =  unsafe {
            match csfml::sfRenderWindow_waitEvent(self.renderWindow, &self.event) {
                0       => false,
                _       => true
            }
        };
        if haveEvent == false {
            return event::NoEvent;
        }
        self.get_wrapped_event()
    }
    
    fn get_wrapped_event(&self) ->event::Event {
        match self.event.typeEvent as c_uint {
            0   => event::Closed,
            1   => event::Resized{width : self.event.p1 as int, height : self.event.p2 as int},
            2   => event::LostFocus,
            3   => event::GainedFocus,
            4   => event::TextEntered{code : self.event.p1 as char},
            5   => {
                let al : bool = match self.event.p2 {
                    0 => false,
                    _ => true
                };
                let ct : bool = match self.event.p3 as int{
                    0 => false,
                    _ => true
                };
                let sh : bool = match self.event.p4  {
                    0 => false,
                    _ => true
                };
                let sy : bool = match self.event.p5 {
                    0 => false,
                    _ => true
                };
                let k : keyboard::Key = unsafe {cast::transmute(self.event.p1 as int)};
                event::KeyPressed{code : k, alt : al, ctrl : ct, shift :sh, system : sy}
            },
            6   => {
                let al : bool = match self.event.p2 {
                    0 => false,
                    _ => true
                };
                let ct : bool = match self.event.p3 as int{
                    0 => false,
                    _ => true
                };
                let sh : bool = match self.event.p4  {
                    0 => false,
                    _ => true
                };
                let sy : bool = match self.event.p5 {
                    0 => false,
                    _ => true
                };
                let k : keyboard::Key = unsafe {cast::transmute(self.event.p1 as int)};
                event::KeyReleased{code : k, alt : al, ctrl : ct, shift :sh, system : sy}
            },
            7   => event::MouseWheelMoved{delta : self.event.p1 as int, x : self.event.p2 as int, y : self.event.p3 as int},
            8   => {
                let button : mouse::MouseButton = unsafe {cast::transmute(self.event.p1 as int)};
                event::MouseButtonPressed{button : button, x : self.event.p2 as int, y : self.event.p3 as int}
            },
            9   => {
                let button : mouse::MouseButton = unsafe {cast::transmute(self.event.p1 as int)};
                event::MouseButtonReleased{button : button, x : self.event.p2 as int, y : self.event.p3 as int}
            },
            10  => event::MouseMoved{x : self.event.p1 as int, y : self.event.p2 as int},
            11  => event::MouseEntered,
            12  => event::MouseLeft,
            13  => event::JoystickButtonPressed{joystickid : self.event.p1 as int, button : self.event.p2 as int},
            14  => event::JoystickButtonReleased{joystickid : self.event.p1 as int, button : self.event.p2 as int},
            15  => {
                let ax : joystick::Axis = unsafe {cast::transmute(self.event.p2 as int)};
                event::JoystickMoved{joystickid : self.event.p1 as uint, axis : ax, position : self.event.p3 as float}
            },
            16  => event::JoystickConnected{joystickid : self.event.p1 as uint},
            17  => event::JoystickDisconnected{joystickid : self.event.p1 as uint},
            _ => event::NoEvent
        }
    }
    
    /// Method close for class RenderWindow. Close the window and destroy attached ressources.
    fn close(&self) -> () {
        unsafe {
            csfml::sfRenderWindow_close(self.renderWindow);
        }
    }

    /**
    *   Method is_open. Verifiy if the Renderwindow is already open.
    */
    fn is_open(&self) -> bool {
        let tmp : sfBool;
        unsafe {
            tmp = csfml::sfRenderWindow_isOpen(self.renderWindow);
        }
        match tmp {
            0 => false,
            _ => true
        }
    }
    
    /**
    *   Method for class RenderWindow, display the content of the window.
    */
    fn display(&self) -> () {
        unsafe {
            csfml::sfRenderWindow_display(self.renderWindow)
        }
    }

     /**
    *   Method for class RenderWindow, set the maximal framerate of the RenderWindow.
    */
    fn set_framerate_limit(&self, limit : uint) -> () {
        unsafe {
            csfml::sfRenderWindow_setFramerateLimit(self.renderWindow, limit as c_uint)
        }
    }
    
    /**
    *   Method for class RenderWindow, get the window OpenGl context settings.
    */
    fn get_settings(&self) -> ContextSettings {
        unsafe {csfml::sfRenderWindow_getSettings(self.renderWindow)}
    }

    /**
    *   Method for class RenderWindow, set the RenderWindow title.
    */
    fn set_title(&self, title : ~str) -> () {
        do str::as_c_str(title) |title_buf| {
            unsafe {
                csfml::sfRenderWindow_setTitle(self.renderWindow, title_buf);
            }
        }
    }
    
    /**
    *   Method for class RenderWindow, display or not the Renderindow.
    */
    fn set_visible(&self, visible : bool) -> () {
        let tmp : sfBool =
            match visible {
                true    => 1,
                false   => 0
            };
        unsafe {
            csfml::sfRenderWindow_setVisible(self.renderWindow, tmp);
        }
    }

    /**
    *   Method for class RenderWindow, set visible the mouse cursor on the RenderWindow.
    */
    fn set_mouse_cursor_visible(&self, visible : bool) -> () {
        let tmp : sfBool =
            match visible {
                true    => 1,
                false   => 0
            };
        unsafe {
            csfml::sfRenderWindow_setMouseCursorVisible(self.renderWindow, tmp);
        }
    }
    
    /**
    *   Method for class RenderWindow, enable or diseable the vertical sync.
    */
    fn set_vertical_sync_enabled(&self, enabled : bool) -> () {
        let tmp : sfBool =
            match enabled {
                true    => 1,
                false   => 0
            };
        unsafe {
            csfml::sfRenderWindow_setVerticalSyncEnabled(self.renderWindow, tmp);
        }
    }

    /**
    *   Method for class RenderWindow, enable or diseable the key repeat.
    */
    fn set_key_repeat_enabled(&self, enabled : bool) -> () {
        let tmp : sfBool =
            match enabled {
                true    => 1,
                false   => 0
            };
        unsafe {
            csfml::sfRenderWindow_setKeyRepeatEnabled(self.renderWindow, tmp);
        }
    }
    
    fn set_active(&self, enabled : bool) -> bool {
        let tmp : sfBool =
            match enabled {
                true    => 1,
                false   => 0
            };
        let res : sfBool = unsafe {
            csfml::sfRenderWindow_setActive(self.renderWindow, tmp)
        };
        match res {
            1   => true,
            _   => false
        }
    }

    /**
    *   Method for class RenderWindow, set the joystick Threshold.
    */
    fn set_joystick_threshold(&self, threshold : float) -> () {
        unsafe {
            csfml::sfRenderWindow_setJoystickThreshold(self.renderWindow, threshold as c_float)
        }
    }

    /**
    *   Method for class RenderWindow, get the position of the RenderWindow on a Vector2i.
    */
    fn get_position(&self) -> vector2::Vector2i {
        unsafe {
            vector2::wrap_vector2i(csfml::sfRenderWindow_getPosition(self.renderWindow))
        }
    }

    /**
    *   Method for class Renderindow, set the position of the Renderindow with a Vector2i.
    */
    fn set_position(&self, position : &vector2::Vector2i) -> () {
        unsafe {
            csfml::sfRenderWindow_setPosition(self.renderWindow, vector2::unwrap_vector2i(position))
        }
    }
    
    /**
    * Method for class RenderWindow, get the size of the RenderWindow on a Vector2u.
    */
    fn get_size(&self) -> vector2::Vector2u {
        unsafe {
            vector2::wrap_vector2u(csfml::sfRenderWindow_getSize(self.renderWindow))
        }
    }
    
    /**
    *   Method for class RenderWindow, set the size of the RenderWindow with a Vector2u
    */
    fn set_size(&self, size : &vector2::Vector2u) -> () {
        unsafe {
            csfml::sfRenderWindow_setSize(self.renderWindow, vector2::unwrap_vector2u(size))
        }
    }
    
    /**
    * Save the current OpenGL render states and matrices
    */
    fn push_GL_states(&self) -> () {
        unsafe {csfml::sfRenderWindow_pushGLStates(self.renderWindow)}
    }

    /**
    * Restore the previously saved OpenGL render states and matrices
    */
    fn pop_GL_states(&self) -> () {
        unsafe {csfml::sfRenderWindow_popGLStates(self.renderWindow)}
    }

    /**
    * Reset the internal OpenGL states so that the target is ready for drawing
    */
    fn reset_GL_states(&self) -> () {
        unsafe {csfml::sfRenderWindow_resetGLStates(self.renderWindow)}
    }

    /**
    * Get the current position of the mouse relatively to a render-window
    */
    fn get_mouse_position(&self) -> vector2::Vector2i {
        unsafe {
            vector2::wrap_vector2i(csfml::sfMouse_getPositionRenderWindow(self.renderWindow))
        }
    }

    /**
    * Set the current position of the mouse relatively to a render-window
    */
    fn set_mouse_position(&self, position : &vector2::Vector2i) -> () {
        unsafe {
            csfml::sfMouse_setPositionRenderWindow(vector2::unwrap_vector2i(position), self.renderWindow)
        }
    }

    /**
    * Drawing functions
    */
    fn draw<T : drawable::Drawable>(&self, t : @T) -> () {
        t.draw_in_render_window(self);
    }

    /// Draw Text
    fn draw_text(&self, text : &Text) -> () {
        unsafe {
            csfml::sfRenderWindow_drawText(self.renderWindow, text.unwrap_text(), ptr::null())
        }
    }

    fn draw_sprite(&self, sprite : &Sprite) -> () {
        unsafe {
            csfml::sfRenderWindow_drawSprite(self.renderWindow, sprite.unwrap_sprite(), ptr::null())
        }
    }

    fn draw_circle_shape(&self, circleShape : &CircleShape) -> () {
        unsafe {
            csfml::sfRenderWindow_drawCircleShape(self.renderWindow, circleShape.unwrap_circle_shape(), ptr::null())
        }
    }

    fn draw_rectangle_shape(&self, rectangleShape : &RectangleShape) -> () {
        unsafe {
            csfml::sfRenderWindow_drawRectangleShape(self.renderWindow, rectangleShape.unwrap_rectangle_shape(), ptr::null())
        }
    }


    /// Clear window with the given color
    fn clear(&self, color : &color::Color) -> () {
        unsafe {
            csfml::sfRenderWindow_clear(self.renderWindow, color.unwrap_color())
        }
    }
    
    fn capture(&self) -> image::Image {
        image::Image::wrap_image(unsafe {csfml::sfRenderWindow_capture(self.renderWindow)})
    }
    
    fn set_view(&self, view : &view::View) -> () {
        unsafe {
            csfml::sfRenderWindow_setView(self.renderWindow, view.unwrap_view())
        }
    }
    
    fn get_view(&self) -> view::View {
        view::View::wrap_view(unsafe {csfml::sfRenderWindow_getView(self.renderWindow)})
    }
    
    fn get_default_view(&self) -> view::View {
        view::View::wrap_view(unsafe {csfml::sfRenderWindow_getDefaultView(self.renderWindow)})
    }
    
    fn map_pixel_to_coords(&self, point : &vector2::Vector2i, view : &view::View) -> vector2::Vector2f {
        unsafe {
            vector2::wrap_vector2f(csfml::sfRenderWindow_mapPixelToCoords(self.renderWindow, vector2::unwrap_vector2i(point), view.unwrap_view()))
        }
    }

    fn map_coords_to_pixel(&self, point : &vector2::Vector2f, view : &view::View) -> vector2::Vector2i {
        unsafe {
            vector2::wrap_vector2i(csfml::sfRenderWindow_mapCoordsToPixel(self.renderWindow, vector2::unwrap_vector2f(point), view.unwrap_view()))
        }
    }

    #[doc(hidden)]
    fn get_sfRenderWindow(&self) -> *csfml::sfRenderWindow {
        self.renderWindow
    }
}

impl Drop for RenderWindow {
    /**
    *   Destructor for class RenderWindow. Destroy all the ressource.
    */
    fn finalize(&self) {
        unsafe {
            csfml::sfRenderWindow_destroy(self.renderWindow);
        }
    }
}