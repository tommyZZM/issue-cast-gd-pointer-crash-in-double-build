/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot::prelude::*;
use godot::classes::{
    MeshInstance3D,
    IMeshInstance3D,
    EditorPlugin,
    IEditorPlugin,
    EditorInspectorPlugin,
    IEditorInspectorPlugin
};

struct IssueReproduceExtensionLibrary;

#[gdextension]
unsafe impl ExtensionLibrary for IssueReproduceExtensionLibrary {}

#[derive(GodotClass)]
#[class(base=MeshInstance3D)]
pub struct MyCustomMesh {
    pub base: Base<MeshInstance3D>,

    #[export]
    size: i32,
}

#[godot_api]
impl IMeshInstance3D for MyCustomMesh {
    // in editor created
    fn init(base: Base<MeshInstance3D>) -> Self {
        // godot_print!("init.. {}", !randInt!());
        godot_print!("init..");
        
        MyCustomMesh {
            base,
            size: 1024
        }
    }
}

#[derive(GodotClass)]
#[class(tool, init, editor_plugin, base=EditorPlugin)]
pub struct MyEditorPlugin {
    base: Base<EditorPlugin>,
    inspector_1: Option<Gd<EditorInspectorPlugin>>
}

#[godot_api]
impl IEditorPlugin for MyEditorPlugin {
    fn enter_tree(&mut self) {
        // Perform typical plugin operations here.
        let inspector_1 = Gd::from_init_fn(|base| {
            MyEdtiorInspectorPlugin { 
                base
            }
        }).clone();

        self.inspector_1 = Some(inspector_1.clone().upcast());

        self.base_mut().add_inspector_plugin(inspector_1.clone().upcast());
    }

    fn exit_tree(&mut self) {
        // Perform typical plugin operations here.
        if let Some(ins) = self.inspector_1.clone() {
            self.base_mut().remove_inspector_plugin(ins.clone())
        } else {
        }
    }
}

#[derive(GodotClass)]
#[class(tool, init, base=EditorInspectorPlugin)]
pub struct MyEdtiorInspectorPlugin{
    base: Base<EditorInspectorPlugin>,
}

#[godot_api]
impl IEditorInspectorPlugin for MyEdtiorInspectorPlugin {
    fn can_handle(&self, object: Gd<godot::classes::Object>) -> bool {
        let a: String = object.get_class().to_string();

        let bool = a == "MyCustomMesh";

        bool
    }

    fn parse_begin(&mut self, object: Gd<godot::classes::Object>,) {
        if let Ok(mesh) = object.try_cast::<MyCustomMesh>() {
            // mesh.bind().size // crash by this
            godot_print!("size is {}", mesh.bind().size) // <-- crash this line
        }
    }
}