use crate::{
    components::*,
    data::{
        CurrentUser, LoginName, Password, RegisterUserBundle, SessionToken, TheWorld, User, UserID,
        UserPicture,
    },
    nav::Route,
};
use bevy_ecs::prelude::*;
use dioxus::prelude::*;

const DEFAULT_AVATAR: Asset = asset!("/assets/Default Avatar.png");

#[derive(Default, Debug, Clone)]
enum AfterOption {
    SignedUp,
    AlreadyExist,
    WrongEither,
    RequireInput,
    #[default]
    None,
}

impl AfterOption {
    pub fn color(&self) -> &str {
        match self {
            AfterOption::SignedUp => "blue",
            AfterOption::AlreadyExist => "blue",
            AfterOption::WrongEither => "red",
            AfterOption::RequireInput => "red",
            AfterOption::None => "",
        }
    }

    pub fn text(&self) -> &str {
        match self {
            AfterOption::SignedUp => "Signed up",
            AfterOption::AlreadyExist => "Already exist",
            AfterOption::WrongEither => "Wrong name or password",
            AfterOption::RequireInput => "Require name and password",
            AfterOption::None => "",
        }
    }
}

#[component]
pub fn Login() -> Element {
    let mut open = use_signal(|| false);
    let mut name = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut the_world = use_context::<Signal<TheWorld>>();
    let mut wrong = use_signal(AfterOption::default);
    let mut session_token = use_context::<Signal<SessionToken>>();

    let toggle = move |_| {
        open.set(true);
    };

    use_effect(move || {
        let _ = open.read();
        name.clear();
        password.clear();
    });

    let current_user = use_memo(move || {
        let world = the_world.read();
        let get_name = world
            .try_query_filtered::<(&Name, &UserID), With<CurrentUser>>()
            .map(|mut q| q.single(&world).ok())
            .flatten();
        get_name.map(|(v, id)| (v.to_string(), id.0))
    });

    let op_user_avatar = use_memo(move || {
        let world = the_world.read();

        let op_avatar = world
            .try_query_filtered::<&UserPicture, With<CurrentUser>>()
            .map(|mut q| q.single(&world).ok().cloned())
            .flatten();

        op_avatar
    });

    let log_out = move |_| {
        let mut world = the_world.write();
        let ent = world
            .query_filtered::<Entity, With<CurrentUser>>()
            .single(&world)
            .unwrap();
        let mut ent_mut = world.entity_mut(ent);
        session_token.write().0.clear();
        ent_mut.remove::<(CurrentUser, SessionToken)>();
    };

    let login = move |_| {
        let mut r_name = name.write();
        let mut r_password = password.write();
        let mut world = the_world.write();
        let mut q_user = world.query::<(Entity, &LoginName, &Password)>();
        let find_it = q_user
            .iter(&world)
            .find(|(_, login_name, pw)| {
                login_name.eq(r_name.as_str()) && pw.eq(r_password.as_str())
            })
            .map(|v| v.0);

        if let Some(ent) = find_it {
            // TODO! SAFETY: SHOULD ADD HASH COLLISION
            let mut buf = [0u8; 16];
            getrandom::fill(&mut buf).unwrap();
            let token = buf.iter().map(|a| a.to_string()).collect::<String>();
            session_token.write().0 = token.clone();
            world
                .entity_mut(ent)
                .insert((CurrentUser, SessionToken(token)));
            open.set(false);
            wrong.set(AfterOption::None);
            r_name.clear();
            r_password.clear();
        } else {
            wrong.set(AfterOption::WrongEither);
        }
    };

    let sign_up = move |_| {
        let mut r_name = name.write();
        let mut r_password = password.write();

        if r_name.is_empty() || r_password.is_empty() {
            wrong.set(AfterOption::RequireInput);
            return;
        }

        let mut world = the_world.write();
        let mut q_user = world.query::<&LoginName>();

        let find_it = q_user
            .iter(&world)
            .any(|login_name| login_name.eq(r_name.as_str()));

        if find_it {
            wrong.set(AfterOption::AlreadyExist);
        } else {
            world.spawn(RegisterUserBundle {
                user: User,
                name: Name::new(r_name.to_owned()),
                login_name: LoginName(std::mem::take(&mut *r_name)),
                password: Password(std::mem::take(&mut *r_password)),
            });
            wrong.set(AfterOption::SignedUp);
            open.set(false);
        }
    };

    rsx! {
        if current_user().is_some() {
            div { class: "div_button", margin_right: "1rem", onclick: toggle,
                div { class: Styles::dx_avatar_item,
                    if let Some(UserPicture(a)) = op_user_avatar() {
                        ImageAvatar {
                            size: AvatarImageSize::Medium,
                            src: a,
                            alt: "User avatar",
                            aria_label: "Avatar",
                        }
                    } else {
                        ImageAvatar {
                            size: AvatarImageSize::Medium,
                            src: DEFAULT_AVATAR,
                            alt: "User avatar",
                            aria_label: "Avatar",
                        }
                    }
                }
            }
        } else {
            Button { variant: ButtonVariant::Primary, onclick: toggle, "Sign In" }
        }
        Sheet {
            open: open(),
            on_open_change: move |v| open.set(v),
            "data-side": SheetSide::Right.as_str(),
            SheetHeader {
                SheetTitle { "Settings" }
                SheetDescription { "Hi, this is Side panel." }
            }

            div {
                display: "grid",
                flex: "1 1 0%",
                grid_auto_rows: "min-content",
                gap: "1.5rem",
                padding: "0 1rem",

                if let Some((user_name, id)) = current_user() {
                    div { display: "grid", gap: "0.75rem",
                        Label { html_for: "greet", "Hello {user_name}" }
                        Link { to: Route::ProfileView { id: id }, "Profile" }
                    }
                } else {
                    div { display: "grid", gap: "0.75rem",
                        Label { html_for: "name", "Name" }
                        Input {
                            id: "name",
                            onchange: move |e: FormEvent| name.set(e.value()),
                        }
                    }
                    div { display: "grid", gap: "0.75rem",
                        Label { html_for: "password", "Password" }
                        Input {
                            id: "password",
                            r#type: "password",
                            onchange: move |e: FormEvent| password.set(e.value()),
                        }
                    }

                    Label { html_for: "after_option", color: wrong().color(), "{wrong().text()}" }
                }

            }

            SheetFooter {
                if current_user().is_some() {
                    Button { variant: ButtonVariant::Destructive, onclick: log_out, "Log Out" }
                } else {
                    Button { onclick: login, "Login" }
                    Button { onclick: sign_up, "Sign Up" }
                }
                SheetClose {
                    r#as: |attributes| rsx! {
                        Button { variant: ButtonVariant::Outline, attributes, "Cancel" }
                    },
                }
            }
            SheetContentClose {}
        }
    }
}
