#![allow(dead_code)]

use restman_rs::request_part;

// URL settings
pub trait HasMediaID {
    fn media_id(&self) -> &str;
}
pub trait HasLessonDescriptor {
    fn lesson_descriptor(&self) -> &str;
}
pub trait HasSectionID {
    fn section_id(&self) -> &str;
}

// url bits
request_part!(User, "user", ());
request_part!(Section, "section", (), HasSectionID, section_id);
request_part!(Lesson, "lesson", (), HasLessonDescriptor, lesson_descriptor);

// api - /api/ui/library/medias
request_part!(Api, "api", ());
request_part!(Ui, "ui", Api);
request_part!(Library, "library", Ui);
request_part!(Medias, "medias", Library);
