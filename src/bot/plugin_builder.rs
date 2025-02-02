use super::ApiMpsc;
use super::{runtimebot::RuntimeBot, Bot};
use event::{AllMsgEvent, AllNoticeEvent, AllRequestEvent, Event};
use std::sync::{Arc, RwLock};
use std::{net::IpAddr, sync::mpsc};


pub mod event;

pub type ListenFn = Arc<dyn Fn(&Event) + Send + Sync + 'static>;


#[derive(Clone)]
pub struct Plugin {
    pub name: String,
    pub all_listen: Vec<Listen>,
}

#[derive(Copy, Clone)]
pub enum OnType {
    OnMsg,
    OnAdminMsg,
    OnAllNotice,
    OnAllRequest,
}

#[derive(Clone)]
pub struct Listen {
    pub on_type: OnType,
    pub handler: ListenFn,
}

pub struct PluginBuilder {
    pub name: String,
    pub host: IpAddr,
    pub port: u16,

    bot: Arc<RwLock<Bot>>,
    api_tx: mpsc::Sender<ApiMpsc>,
}

impl PluginBuilder {
    pub fn new(name: String, bot: Arc<RwLock<Bot>>, api_tx: mpsc::Sender<ApiMpsc>) -> Self {
        let (host, port) = {
            let bot_lock = bot.read().unwrap();
            (
                bot_lock.information.server.host,
                bot_lock.information.server.port,
            )
        };
        {
            let bot = bot.clone();
            let mut bot_lock = bot.write().unwrap();
            bot_lock.plugins.push(Plugin {
                name: name.clone(),
                all_listen: Vec::new(),
            });
        }
        PluginBuilder {
            name,
            bot,
            host,
            port,
            api_tx,
        }
    }

    pub fn build_runtime_bot(&self) -> RuntimeBot {
        RuntimeBot {
            main_admin: self.bot.read().unwrap().information.main_admin,
            admin: self.bot.read().unwrap().information.admin.clone(),
            host: self.host,
            port: self.port,
            api_tx: self.api_tx.clone(),
        }
    }
}

impl PluginBuilder {
    /// 注册消息处理函数。
    ///
    /// 注册一个处理程序（handler），用于处理接收到的消息事件（`AllMsgEvent`）。
    /// 接收闭包，要求函数接受 `AllMsgEvent` 类型的参数，并返回 `Result` 类型。
    /// 闭包必须实现 `Send` 、 `Sync`和 `'static`，因为要保证多线程安全以及在确保闭包在整个程序生命周期有效。
    pub fn on_msg<F>(&mut self, handler: F)
    where
        F: Fn(&AllMsgEvent) + Send + Sync + 'static,
    {
        let bot = self.bot.clone();
        for plugin in &mut bot.write().unwrap().plugins {
            if plugin.name != self.name {
                continue;
            }

            plugin.all_listen.push(Listen {
                on_type: OnType::OnMsg,
                handler: Arc::new(move |event| {
                    if let Event::OnMsg(e) = event {
                        handler(e)
                    } else {
                        panic!()
                    }
                }),
            });
            return;
        }
    }

    /// 注册消息处理函数。
    ///
    /// 注册一个处理程序（handler），用于处理接收到的消息事件（`AllMsgEvent`）。
    /// 接收闭包，要求函数接受 `AllMsgEvent` 类型的参数，并返回 `Result` 类型。
    /// 闭包必须实现 `Send` 、 `Sync`和 `'static`，因为要保证多线程安全以及在确保闭包在整个程序生命周期有效。
    pub fn on_admin_msg<F>(&mut self, handler: F)
    where
        F: Fn(&AllMsgEvent) + Send + Sync + 'static,
    {
        let bot = self.bot.clone();
        for plugin in &mut bot.write().unwrap().plugins {
            if plugin.name != self.name {
                continue;
            }
            plugin.all_listen.push(Listen {
                on_type: OnType::OnAdminMsg,
                handler: Arc::new(move |event| {
                    if let Event::OnMsg(e) = event {
                        handler(e)
                    } else {
                        panic!()
                    }
                }),
            });
            return;
        }
    }

    /// 注册消息处理函数。
    ///
    /// 注册一个处理程序（handler），用于处理接收到的消息事件（`AllNoticeEvent`）。
    /// 接收闭包，要求函数接受 `AllNoticeEvent` 类型的参数，并返回 `Result` 类型。
    /// 闭包必须实现 `Send` 、 `Sync`和 `'static`，因为要保证多线程安全以及在确保闭包在整个程序生命周期有效。
    pub fn on_all_notice<F>(&mut self, handler: F)
    where
        F: Fn(&AllNoticeEvent) + Send + Sync + 'static,
    {
        let bot = self.bot.clone();
        for plugin in &mut bot.write().unwrap().plugins {
            if plugin.name != self.name {
                continue;
            }
            plugin.all_listen.push(Listen {
                on_type: OnType::OnAllNotice,
                handler: Arc::new(move |event| {
                    if let Event::OnAllNotice(e) = event {
                        handler(e)
                    } else {
                        panic!()
                    }
                }),
            });
            return;
        }
    }

    /// 注册消息处理函数。
    ///
    /// 注册一个处理程序（handler），用于处理接收到的消息事件（`AllRequestEvent`）。
    /// 接收闭包，要求函数接受 `AllRequestEvent` 类型的参数，并返回 `Result` 类型。
    /// 闭包必须实现 `Send` 、 `Sync`和 `'static`，因为要保证多线程安全以及在确保闭包在整个程序生命周期有效。
    pub fn on_all_request<F>(&mut self, handler: F)
    where
        F: Fn(&AllRequestEvent) + Send + Sync + 'static,
    {
        let bot = self.bot.clone();
        for plugin in &mut bot.write().unwrap().plugins {
            if plugin.name != self.name {
                continue;
            }
            plugin.all_listen.push(Listen {
                on_type: OnType::OnAllRequest,
                handler: Arc::new(move |event| {
                    if let Event::OnAllRequest(e) = event {
                        handler(e)
                    } else {
                        panic!()
                    }
                }),
            });
            return;
        }
    }
}
