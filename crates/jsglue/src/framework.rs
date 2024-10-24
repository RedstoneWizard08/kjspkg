use once_cell::sync::Lazy;
use regex::Regex;

pub static VITE_HMR_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^\d+:\d+:\d+ (A|P)M \[vite\] hmr").unwrap());

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Framework {
    None,

    /// By default this path is `"/_next/webpack-hmr"`. If you changed it,
    /// use the `Custom` preset with no protocol value.
    Next,

    /// Nuxt (at the time of writing, iirc) is Vite-based. It uses the Vite
    /// preset's defaults, however I can't remember what the HMR path is.
    Nuxt,

    /// In your Vite config, make sure to set `server.hmr.path`
    /// to `"/svelte-hmr"` in order for this to work.
    SvelteKit,

    /// Note that Vite will require you to set the `server.hmr.path`
    /// value to what you put here in order to proxy the HMR WebSocket.
    Vite(&'static str),

    /// Note that you will need to change the HMR path to what you put
    /// here in order to proxy the HMR WebSocket.
    ///
    /// The second value here is the HMR protocol. This is an optional
    /// value, as most times it will just be `None`. An example of this
    /// would be Vite, which uses the value `"vite-hmr"`.
    Custom(&'static str, Option<&'static str>),
}

impl Framework {
    pub fn get_hmr_path(self) -> &'static str {
        match self {
            Framework::None => "",
            Framework::Next => "/_next/webpack-hmr",
            Framework::Nuxt => todo!(),
            Framework::SvelteKit => "/svelte-hmr",
            Framework::Vite(val) => val,
            Framework::Custom(val, _) => val,
        }
    }

    pub fn get_subprotocol(self) -> Option<&'static str> {
        match self {
            Framework::Next => None,
            Framework::None => None,
            Framework::Nuxt | Framework::SvelteKit | Framework::Vite(_) => Some("vite-hmr"),
            Framework::Custom(_, val) => val,
        }
    }

    pub fn get_ready_str(self) -> &'static str {
        match self {
            Framework::None => "",
            Framework::Next | Framework::Custom(_, _) => "ready in ",
            Framework::Nuxt | Framework::SvelteKit | Framework::Vite(_) => {
                "press h + enter to show help"
            }
        }
    }

    pub fn process_message(self, msg: &str) {
        match self {
            Framework::Nuxt | Framework::SvelteKit | Framework::Vite(_) => {
                if VITE_HMR_REGEX.is_match(msg) {
                    info!("{}", VITE_HMR_REGEX.replace(msg, "hmr"));
                } else {
                    info!("{}", msg);
                }
            }

            _ => info!("{}", msg),
        }
    }
}
