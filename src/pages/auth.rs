use crate::auth::{LoginAction, LoginSignal, SignupAction, SignupSignal};
use crate::utils::classnames;
use leptos::*;
use leptos_meta::Title;
use leptos_router::ActionForm;
#[component]

// is_login -> not active
// is_not_login -> active
pub fn Auth() -> impl IntoView {
    let (is_login, set_is_login) = create_signal(true);
    let signup: SignupSignal = create_server_action::<SignupAction>();
    let login: LoginSignal = create_server_action::<LoginAction>();
    view! {
      <Title text="Login or Sign up" />
      <div>
        <div class="flex gap-3 items-center mb-5 text-4xl font-bold">
          <div class="ping-logo"></div>
          <h1>Ping</h1>
        </div>
        <p class="text-white">{move || { if is_login() { "true" } else { "false" } }}</p>
        <section class="overflow-hidden relative max-w-full shadow-lg bg-primary-800 rounded-[1.25rem] w-[48rem] min-h-[30rem]">
          <div class=move || classnames(
            &[
              (
                "absolute top-0 left-0 w-1/2 h-full opacity-0 transition-all duration-600 ease-smooth z-[1]",
                true,
              ),
              ("translate-x-full opacity-100 z-[5]", !is_login()),
            ],
          )>
            <ActionForm
              action=signup
              class="flex flex-col justify-center items-center px-10 h-full bg-primary-800"
            >
              <h2 class="text-3xl font-bold">Create an account</h2>
              <p class="my-5">Login With Email & Password</p>
              <input
                class="px-4 my-2 w-full text-black rounded-lg border-none outline-none py-[.625rem] bg-[#eee]"
                placeholder="Enter name"
              />
              <input
                class="px-4 my-2 w-full text-black rounded-lg border-none outline-none py-[.625rem] bg-[#eee]"
                placeholder="Enter email"
              />
              <input
                class="px-4 my-2 w-full text-black rounded-lg border-none outline-none py-[.625rem] bg-[#eee]"
                placeholder="Enter password"
              />
              <button class="px-11 text-xs font-semibold uppercase rounded-lg bg-primary-700 mt-[.625rem] py-[.625rem]">
                Sign Up
              </button>
            </ActionForm>
          </div>

          <div class=move || classnames(
            &[
              (
                "absolute top-0 left-0 w-1/2 h-full transition-[transform] duration-600 ease-smooth z-[2]",
                true,
              ),
              ("translate-x-full", !is_login()),
            ],
          )>
            <ActionForm
              action=login
              class="flex flex-col justify-center items-center px-10 h-full bg-primary-800"
            >
              <h2 class="text-3xl font-bold">Sign In</h2>
              <p class="my-5">Login With Email & Password</p>
              <input
                name="email"
                class="px-4 my-2 w-full text-black rounded-lg border-none outline-none py-[.625rem] bg-[#eee]"
                placeholder="Enter email"
              />
              <input
                class="px-4 my-2 w-full text-black rounded-lg border-none outline-none py-[.625rem] bg-[#eee]"
                name="password"
                placeholder="Enter password"
              />
              <button class="px-11 text-xs font-semibold uppercase rounded-lg bg-primary-700 mt-[.625rem] py-[.625rem]">
                Sign in
              </button>
            </ActionForm>
          </div>

          <div class=move || classnames(
            &[
              (
                "overflow-hidden absolute top-0 left-1/2 w-1/2 h-full duration-600 ease-smooth transition-[transform] will-change-transform rounded-[1.25rem] z-10",
                true,
              ),
              ("-translate-x-full", !is_login()),
            ],
          )>
            <div class=move || classnames(
              &[
                (
                  "relative -left-full h-full bg-primary-700 w-[200%] duration-600 ease-smooth transition-[transform] will-change-transform",
                  true,
                ),
                ("translate-x-1/2", !is_login()),
              ],
            )>
              <div class=move || classnames(
                &[
                  (
                    "flex absolute top-0 flex-col justify-center items-center w-1/2 h-full text-center px-[1.875rem] duration-600 ease-smooth transition-[transform] will-change-transform",
                    true,
                  ),
                  ("-translate-x-full", is_login()),
                ],
              )>
                <h2 class="text-3xl font-bold">Welcome Back to Ping!</h2>
                <p class="my-5 text-sm">
                  Log in to continue your real-time conversations and stay connected.
                </p>
                <button
                  on:click=move |_| set_is_login(!is_login())
                  class="px-11 text-xs font-semibold uppercase rounded-lg border border-white mt-[.625rem] py-[.625rem]"
                >
                  Sign in
                </button>
              </div>
              <div class=move || classnames(
                &[
                  (
                    "flex absolute top-0 right-0 flex-col justify-center items-center w-1/2 h-full text-center px-[1.875rem] duration-600 ease-smooth transition-[transform] will-change-transform",
                    true,
                  ),
                  ("translate-x-full", !is_login()),
                ],
              )>
                <h2 class="text-3xl font-bold">Welcome to Ping!</h2>
                <p class="my-5 text-sm">
                  Join Ping today and start messaging in real-time with ease.
                </p>
                <button
                  on:click=move |_| set_is_login(!is_login())
                  class="px-11 text-xs font-semibold uppercase rounded-lg border border-white mt-[.625rem] py-[.625rem]"
                >
                  Sign up
                </button>
              </div>
            </div>
          </div>
        </section>
      </div>
    }
}
