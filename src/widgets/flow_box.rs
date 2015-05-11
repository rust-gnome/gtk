// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! A container that allows reflowing its children

use ffi;

/// GtkFlowBox — A container that allows reflowing its children
struct_Widget!(FlowBox);

impl FlowBox {
    pub fn new() -> Option<FlowBox> {
        let tmp_pointer = unsafe { ffi::gtk_flow_box_new() };
        check_pointer!(tmp_pointer, FlowBox)
    }
}

impl_drop!(FlowBox);
impl_TraitWidget!(FlowBox);

impl ::ContainerTrait for FlowBox {}
impl ::FlowBoxTrait for FlowBox {}

struct_Widget!(FlowBoxChild);

impl FlowBoxChild {
    pub fn new() -> Option<FlowBoxChild> {
        let tmp_pointer = unsafe { ffi::gtk_flow_box_child_new() };
        check_pointer!(tmp_pointer, FlowBoxChild)
    }
}

impl_drop!(FlowBoxChild);
impl_TraitWidget!(FlowBoxChild);

impl ::ContainerTrait for FlowBoxChild {}
impl ::BinTrait for FlowBoxChild {}
impl ::FlowBoxChildTrait for FlowBoxChild {}
