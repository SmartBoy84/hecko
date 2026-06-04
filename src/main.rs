use restman_rs::{client::sync_client::ApiClient, request::ApiRequest};

use crate::echo::{
    EchoUser,
    config::EchoRequestConfig,
    endpoints::{Enrollments, Syllabus},
};

mod echo;

const MY_ECHO_TOKEN: &str = "05fdceefe090a31fe5121987f2d9dafe179126d0-role=Student&institution=3db867e1-3876-45f2-9651-4e18857c6760&authn=true&roles=Student&csrfToken=1db90e164eb13f2c386964506a3255a8934b5a61-1780037995857-33310617e1a487bdfc2680a0&sessionExpiresAt=2026-05-29T10%3A59%3A55.766Z&user=e360b75a-1d71-4c13-9191-adada30e4b6c&sessionCode=3b3ebbf2-0dc9-4748-ac6c-d5437825044e";

fn main() {
    let echo_user = EchoUser::new(MY_ECHO_TOKEN);

    let course_req = ApiRequest::<Syllabus>::new(
        &EchoRequestConfig::new().section_id("5f4dbdd0-6bc3-4c00-9ef3-17bb593851e6"),
    );
    // println!(
    //     "{:?}",
    //     echo_user.backend().raw_request(&course_req, &[]).unwrap()
    // );

    // ISSUE: request goes through once at the start (for all URLs), then subsequent ones redirect to login page...

    // let enrollments_req = ApiRequest::<Enrollments>::new(&());
    // println!("{}", enrollments_req.uri());
    // let [courses] = echo_user
    //     .request(&ApiRequest::<Enrollments>::new(&()))
    //     .unwrap();

    // let rando_course = &courses.user_sections[0];
    // let lessons_req =
    //     ApiRequest::<Syllabus>::new(&EchoRequestConfig::new().section_id(&rando_course.section_id));
    // println!("{}", lessons_req.uri());
    // println!("{rando_course:?}");
    // //https://echo360.net.au/section/5f4dbdd0-6bc3-4c00-9ef3-17bb593851e6/syllabus
    // //https://echo360.net.au/section/5f4dbdd0-6bc3-4c00-9ef3-17bb593851e6/syllabus

    // // let section_id = courses[0]

    // println!(
    //     "{}",
    //     echo_user.backend().raw_request(&lessons_req, &[]).unwrap()
    // );

    // println!("{:?}", echo_user.request(&enrollments_req));
    // println!("{courses}")

    // println!("Hello, world!");
}
