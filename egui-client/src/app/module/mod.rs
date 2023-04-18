pub(crate) mod home;
pub(crate) mod login;
pub(crate) mod page;

#[derive(Default)]
pub(crate) enum Module {
    #[default]
    Login,
    Home,
}

