pub mod enrollments;
pub mod syllabus;

use restman_rs::{GET, endpoint};

use crate::echo::{
    client::{Echo360, EchoRes},
    endpoints::{enrollments::EnrollmentsRes, syllabus::SyllabusRes},
    request::{Section, User},
};

endpoint!(Echo360, pub Enrollments, "enrollments", User, EchoRes<EnrollmentsRes>, (), (), GET);
endpoint!(Echo360, pub Syllabus, "syllabus", Section, EchoRes<SyllabusRes>, (), (), GET);
