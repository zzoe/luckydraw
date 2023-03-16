pub(crate) mod home;
pub(crate) mod login;

#[derive(Default)]
pub(crate) enum Module {
    #[default]
    Login,
    Home,
}
