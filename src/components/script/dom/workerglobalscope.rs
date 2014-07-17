/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::trace::Untraceable;
use dom::bindings::global;
use dom::bindings::js::{JS, JSRef, Temporary, OptionalSettable};
use dom::bindings::utils::{Reflectable, Reflector};
use dom::console::Console;
use dom::eventtarget::{EventTarget, WorkerGlobalScopeTypeId};

use servo_net::resource_task::ResourceTask;

use js::jsapi::JSContext;
use js::rust::Cx;

use std::cell::Cell;
use std::rc::Rc;

#[deriving(PartialEq,Encodable)]
pub enum WorkerGlobalScopeId {
    DedicatedGlobalScope,
}

#[deriving(Encodable)]
pub struct WorkerGlobalScope {
    pub eventtarget: EventTarget,
    js_context: Untraceable<Rc<Cx>>,
    resource_task: Untraceable<ResourceTask>,
    console: Cell<Option<JS<Console>>>,
}

impl WorkerGlobalScope {
    pub fn new_inherited(type_id: WorkerGlobalScopeId,
                         cx: Rc<Cx>,
                         resource_task: ResourceTask) -> WorkerGlobalScope {
        WorkerGlobalScope {
            eventtarget: EventTarget::new_inherited(WorkerGlobalScopeTypeId(type_id)),
            js_context: Untraceable::new(cx),
            resource_task: Untraceable::new(resource_task),
            console: Cell::new(None),
        }
    }

    pub fn get_cx(&self) -> *mut JSContext {
        self.js_context.ptr
    }

    pub fn resource_task<'a>(&'a self) -> &'a ResourceTask {
        &*self.resource_task
    }
}

pub trait WorkerGlobalScopeMethods {
    fn Self(&self) -> Temporary<WorkerGlobalScope>;
    fn Console(&self) -> Temporary<Console>;
}

impl<'a> WorkerGlobalScopeMethods for JSRef<'a, WorkerGlobalScope> {
    fn Self(&self) -> Temporary<WorkerGlobalScope> {
        Temporary::from_rooted(self)
    }

    fn Console(&self) -> Temporary<Console> {
        if self.console.get().is_none() {
            let console = Console::new(&global::Worker(*self));
            self.console.assign(Some(console));
        }
        Temporary::new(self.console.get().get_ref().clone())
    }
}

impl Reflectable for WorkerGlobalScope {
    fn reflector<'a>(&'a self) -> &'a Reflector {
        self.eventtarget.reflector()
    }
}
