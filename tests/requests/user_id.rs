use insta::assert_debug_snapshot;
use loco_rs::testing::prelude::*;
use serial_test::serial;
use stat_api::app::App;

// use crate::requests::prepare_data;

macro_rules! configure_insta {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        settings.set_snapshot_suffix("user_id_request");
        let _guard = settings.bind_to_scope();
    };
}

#[tokio::test]
#[serial]
async fn can_get_user_id() {
    configure_insta!();
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/api/user_ids/new").await;
        assert_eq!(res.status_code(), 200, "Creation request should succeed");
        insta::with_settings!({
            filters => vec![
                (
                    r"([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12})",
                    "PID",
                ),
                (
                    r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2} \+\d{2}:\d{2}",
                    "DATE",
                ),
            ]
        }, {
            assert_debug_snapshot!(res.text());
        });
    })
    .await;
}

// #[tokio::test]
// #[serial]
// async fn can_get_user_ids() {
//     configure_insta!();
//     request::<App, _, _>(|request, ctx| async move {
//         let user = prepare_data::init_user_login(&request, &ctx).await;
// TODO: create prepare_data for user_id requests
//         let (auth_key, auth_value) = prepare_data::auth_header(&user.token);
//         let response = request
//             .get("/api/user_ids/1")
//             .add_header(auth_key, auth_value)
//             .await;
//         assert_eq!(
//             response.status_code(),
//             200,
//             "Authorization should be succesfull"
//         );
//         // insta::with_settings!({
//         //     filters => vec![
//         //         (
//         //             r"([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12})",
//         //             "PID",
//         //         ),
//         //         (
//         //             r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2} \+\d{2}:\d{2}",
//         //             "DATE",
//         //         ),
//         //     ]
//         // }, {
//         //     assert_debug_snapshot!(res.text());
//         // });
//     })
//     .await;
// }
