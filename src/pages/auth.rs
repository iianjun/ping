use crate::utils::classnames;
use leptos::*;
use leptos_meta::Title;

#[component]

pub fn Auth() -> impl IntoView {
    let (is_login, set_is_login) = create_signal(true);
    view! {
      <Title text="Login or Sign up" />
      <section class="overflow-hidden relative max-w-full shadow-lg bg-primary-800 rounded-[1.25rem] w-[48rem] min-h-[30rem]">
        <div class="absolute top-0 left-0 w-1/2 h-full duration-600 ease-smooth transition-[all] z-[2]">
          <form class="flex flex-col justify-center items-center px-10 h-full">
            <h2 class="text-3xl font-bold">Sign In</h2>
            <p class="my-5">Login With Email & Password</p>
            <input
              class="px-4 my-2 w-full text-black rounded-lg border-none outline-none py-[.625rem] bg-[#eee]"
              placeholder="Enter email"
            />
            <input
              class="px-4 my-2 w-full text-black rounded-lg border-none outline-none py-[.625rem] bg-[#eee]"
              placeholder="Enter password"
            />
            <button
              on:click=move |_| set_is_login(!is_login())
              class="px-11 text-xs font-semibold uppercase rounded-lg bg-primary-700 mt-[.625rem] py-[.625rem]"
            >
              Sign in
            </button>
          </form>
        </div>
        <div class="absolute top-0 left-0 w-1/2 h-full opacity-0 duration-600 ease-smooth transition-[all] z-[1]"></div>
        <div class=move || classnames(
          &[
            (
              "overflow-hidden absolute top-0 left-1/2 z-10 w-1/2 h-full duration-600 ease-smooth transition-[all] rounded-[1.25rem]",
              true,
            ),
            ("-translate-x-full", !is_login()),
          ],
        )>
          <div class=move || classnames(
            &[
              (
                "relative -left-full h-full bg-primary-700 w-[200%] duration-600 ease-smooth transition-[all]",
                true,
              ),
              ("translate-x-0", is_login()),
              ("translate-x-1/2", !is_login()),
            ],
          )>
            <div class=move || classnames(
              &[
                (
                  "flex absolute top-0 flex-col justify-center items-center w-1/2 h-full text-center px-[1.875rem]",
                  true,
                ),
                ("translate-x-[-200%]", is_login()),
                ("translate-x-0", !is_login()),
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
                  "flex absolute top-0 right-0 flex-col justify-center items-center w-1/2 h-full text-center translate-x-0 px-[1.875rem] translate-x-0",
                  true,
                ),
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
    }
}
