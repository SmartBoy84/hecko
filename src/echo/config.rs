#![allow(dead_code)]

// implement general config struct using bon

use std::borrow::Cow;

use restman_rs::request::RequestConfig;

use crate::echo::request::{HasLessonDescriptor, HasMediaID, HasSectionID};

// use bon to create a general config struct
#[derive(Debug, bon::Builder)]
#[builder(builder_type(name = EchoRequestConfig, vis = "pub"), finish_fn(vis = ""))]
struct _EchoRequestConfig<'a> {
    #[builder(into, getter(name = get_section_id_internal, vis = ""))]
    section_id: Cow<'a, str>,

    #[builder(into, getter(name = get_lesson_descriptor_internal, vis = ""))]
    lesson_descriptor: Cow<'a, str>,

    #[builder(into, getter(name = get_media_id_internal, vis = ""))]
    media_id: Cow<'a, str>,
}

// bon is pretty cool!
impl<'a, S: echo_request_config::State> RequestConfig for EchoRequestConfig<'a, S> {}

impl<'a> EchoRequestConfig<'a, echo_request_config::Empty> {
    pub fn new() -> Self {
        _EchoRequestConfig::builder()
    }
}

impl<'a> HasSectionID for EchoRequestConfig<'a, echo_request_config::SetSectionId> {
    fn section_id(&self) -> &str {
        self.get_section_id_internal().as_ref()
    }
}

impl<'a> HasLessonDescriptor for EchoRequestConfig<'a, echo_request_config::SetLessonDescriptor> {
    fn lesson_descriptor(&self) -> &str {
        self.get_lesson_descriptor_internal().as_ref()
    }
}

impl<'a> HasMediaID for EchoRequestConfig<'a, echo_request_config::SetMediaId> {
    fn media_id(&self) -> &str {
        self.get_media_id_internal().as_ref()
    }
}
