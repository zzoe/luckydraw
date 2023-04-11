use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;

use yew::html::{AnyScope, Scope};
use yew::scheduler::Shared;
use yew::Callback;
use yew::{Component, Context};

use crate::AppSys;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Module {
    Root,
    Sys1,
}

#[derive(Clone)]
pub struct AppScope {
    module: Module,
    scope: AnyScope,
}

impl PartialEq for AppScope {
    fn eq(&self, other: &Self) -> bool {
        self.module == other.module
    }
}

impl AppScope {
    pub fn new(module: Module, scope: &Scope<impl Component>) -> Self {
        AppScope {
            module,
            scope: scope.clone().into(),
        }
    }
}

#[derive(Clone, Default, PartialEq)]
pub struct AppContext {
    pub sys: AppSys,
    pub userid: usize,
    pub scopes: HashMap<Module, AppScope>,
}

pub(crate) trait ContextExt {
    fn context(&self) -> Shared<AppContext>;
    fn insert_scope(&self, module: Module);
    fn send<DST: Component>(&self, module: Module, msg: DST::Message);
}

impl<COMP: Component> ContextExt for Context<COMP> {
    fn context(&self) -> Shared<AppContext> {
        let (context, _) = self
            .link()
            .context::<Shared<AppContext>>(Callback::noop())
            .unwrap();
        context
    }

    fn insert_scope(&self, module: Module) {
        if let Some((c, _)) = self.link().context::<Shared<AppContext>>(Callback::noop()) {
            RefCell::borrow_mut(&c)
                .scopes
                .insert(module, AppScope::new(module, self.link()));
        }
    }

    fn send<DST: Component>(&self, module: Module, msg: DST::Message) {
        if let Some((c, _)) = self.link().context::<Shared<AppContext>>(Callback::noop()) {
            if let Some(s) = RefCell::borrow_mut(&c).scopes.get_mut(&module) {
                if TypeId::of::<DST>().eq(s.scope.get_type_id()) {
                    s.scope.clone().downcast::<DST>().send_message(msg);
                    return;
                } else {
                    log::error!("发送的模块类型与注册的模块类型不一致");
                }
            } else {
                log::error!("没找到此模块{module:?}");
            }
        }
        log::error!("往模块{:?}发送消息失败", module);
    }
}
