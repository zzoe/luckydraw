pub(crate) mod home;
pub(crate) mod login;

pub(crate) enum Page {
    Login,
    Home,
}

impl Default for Page{
    fn default() -> Self {
        Page::Login
    }
}
