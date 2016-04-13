use xcb::xproto as xproto;

use wm::window_system::Wm;

// a client wrapping a window
#[derive(Debug)]
pub struct Client {
    pub window: xproto::Window, // the window (a direct child of root)
    urgent: bool,               // is the urgency hint set?
    w_type: xproto::Atom,       // client/window type
    tags: Vec<Tag>,             // all tags this client is visible on
}

impl Client {
    // setup a new client from a window manager for a specific window
    pub fn new(wm: &Wm, window: xproto::Window, tags: Vec<Tag>)
        -> Option<Client> {
        let cookie = wm.get_ewmh_property(window, "_NET_WM_WINDOW_TYPE");
        match cookie.get_reply() {
            Ok(props) => {
                let w_type = props.type_();
                Some(Client {window: window,
                    urgent: false, w_type: w_type, tags: tags})
            },
            Err(_) => {
                None
            }
        }
    }

    // is a client visible on a set of tags?
    fn has_tags(&self, tags: &[Tag]) -> bool {
        for tag in tags {
            if self.tags.contains(tag) {
                return true;
            }
        }
        false
    }
}

// a client list, managing all direct children of the root window
pub struct ClientList {
    clients: Vec<Client>,
}

impl ClientList {
    // initialize an empty client list
    // TODO: decide upon an optional with_capacity() call
    pub fn new() -> ClientList {
        ClientList {clients: Vec::new()}
    }

    // get a list of references of windows that are visible on a set of tags
    pub fn match_clients_by_tags(&self, tags: &[Tag]) -> Vec<&Client> {
        self.clients.iter().filter(|elem| elem.has_tags(tags)).collect()
    }

    // get a client that corresponds to the given window
    pub fn get_client_by_window(&self, window: xproto::Window)
        -> Option<&Client> {
        self.clients.iter().find(|client| client.window == window)
    }

    // add a new client
    pub fn add(&mut self, client: Client) {
        self.clients.push(client);
    }

    pub fn remove(&mut self, window: xproto::Window) {
        if let Some(pos) =
            self.clients.iter().position(|elem| elem.window == window) {
            self.clients.remove(pos);
        }
    }
}

// a set of (symbolic) tags - to be extended/modified
#[derive(Debug, PartialEq, Clone)]
pub enum Tag {
    Foo,
}
