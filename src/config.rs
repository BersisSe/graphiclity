/// Main Config of window <br>
/// Contains logical and physical sizes  <br>
/// Logical size is your drawing canvas size
/// Physical size is the actual window size on screen  
/// Defaults to 800x600 logical and physical sizes
pub struct Config {
    /// Window Title
    pub title: String,
    /// Is window resizeable
    pub resizeable: bool,

    /// Logical resolution Your
    pub logical_width: u32,
    pub logical_height: u32,
    
    // Initial window size
    pub window_width: u32,
    pub window_height: u32,
}
pub struct ConfigBuilder {
    /// Window Title
    title: Option<String>,
    resizeable: Option<bool>,
    /// Logical resolution Your
    logical_width: Option<u32>,
    logical_height: Option<u32>,

    // Initial window size
    window_width: Option<u32>,
    window_height: Option<u32>,
}
impl ConfigBuilder {
    /// Set the window title
    pub fn with_title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }
    /// Set the window size (physical size)
    pub fn set_window_size(mut self, size: (u32, u32)) -> Self {
        self.window_width = Some(size.0);
        self.window_height = Some(size.1);
        self
    }
    /// Set the logical size (drawing canvas size)
    pub fn set_logical_size(mut self, size: (u32, u32)) -> Self {
        self.logical_width = Some(size.0);
        self.logical_height = Some(size.1);
        self
    }
    pub fn set_resizeable(mut self, resizeable: bool) -> Self {
        self.resizeable = Some(resizeable);
        self
    }
    /// Build the Config
    pub fn build(self) -> Config {
        Config {
            title: self.title.unwrap_or("Graphiclity Window".to_string()),
            resizeable: self.resizeable.unwrap_or(true),
            logical_width: self.logical_width.unwrap_or(800),
            logical_height: self.logical_height.unwrap_or(600),
            window_width: self.window_width.unwrap_or(800),
            window_height: self.window_height.unwrap_or(600),
        }
    }
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder {
            logical_height: None,
            logical_width: None,
            window_height: None,
            window_width: None,
            resizeable: None,
            title: None,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            title: "Graphiclity Window".to_string(),
            resizeable: true,
            logical_width: 800,
            logical_height: 600,
            window_width: 800,
            window_height: 600,
        }
    }
}
