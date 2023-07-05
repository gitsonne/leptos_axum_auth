
use crate::error_template::{AppError, ErrorTemplate};
use crate::todo::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/Leptos_Axum_Auth.css"/>

        // sets the document title
        <Title text="Leptos Login"/>

        // content for this welcome page
        <Router fallback=|cx| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { cx,
                <ErrorTemplate outside_errors/>
            }
            .into_view(cx)
        }>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <SignInPage/> }/>
                    <Route path="/signup" view=|cx| view! { cx, <SignUpPage/> }/>
                    <Route path="/resetpassword" view=|cx| view! { cx, <ResetPasswordPage/> }/>
                    <Route path="/todo" view=|cx| view! { cx, <Todos/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn SignInPage(cx: Scope) -> impl IntoView {

    view! { cx,

        <div class="flex items-center justify-center h-screen bg-gray-100">
        <div class="w-80 bg-white rounded shadow p-6">
          <h2 class="text-2xl font-semibold mb-6">"Sign In"</h2>
          <form>
            <div class="mb-4">
              <label for="email" class="block text-gray-700 font-semibold mb-2">"Email"</label>
              <input id="email" type="email" name="email" placeholder="Enter your email"
                class="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:border-indigo-500"></input>
            </div>
            <div class="mb-6">
              <label for="password" class="block text-gray-700 font-semibold mb-2">"Password"</label>
              <input id="password" type="password" name="password" placeholder="Enter your password"
                class="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:border-indigo-500"></input>
            </div>
            <div class="flex items-center justify-between mb-4">
              <button type="submit"
                class="bg-indigo-500 hover:bg-indigo-600 text-white font-semibold py-2 px-4 rounded focus:outline-none focus:shadow-outline">
                "Sign In"
              </button>
              <a href="/signup" class="text-gray-600 hover:text-indigo-500">"Sign Up"</a>
            </div>
            <a href="/resetpassword" class="text-gray-600 hover:text-indigo-500">"Forgot password?"</a>
          </form>
        </div>
      </div>      
    }
}


#[component]
fn SignUpPage(cx: Scope) -> impl IntoView {

    view! { cx,

        <div class="flex items-center justify-center h-screen bg-gray-100">
        <div class="w-80 bg-white rounded shadow p-6">
          <h2 class="text-2xl font-semibold mb-6">"Sign Up"</h2>
          <form>
            <div class="mb-4">
              <label for="name" class="block text-gray-700 font-semibold mb-2">"Name"</label>
              <input id="name" type="text" name="name" placeholder="Enter your name"
                class="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:border-indigo-500"></input>
            </div>
            <div class="mb-4">
              <label for="email" class="block text-gray-700 font-semibold mb-2">"Email"</label>
              <input id="email" type="email" name="email" placeholder="Enter your email"
                class="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:border-indigo-500"></input>
            </div>
            <div class="mb-6">
              <label for="password" class="block text-gray-700 font-semibold mb-2">"Password"</label>
              <input id="password" type="password" name="password" placeholder="Enter your password"
                class="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:border-indigo-500"></input>
            </div>
            <div class="flex items-center justify-between">
              <button type="submit"
                class="bg-indigo-500 hover:bg-indigo-600 text-white font-semibold py-2 px-4 rounded focus:outline-none focus:shadow-outline">
                "Sign Up"
              </button>
              <a href="/" class="text-gray-600 hover:text-indigo-500">"Sign In"</a>
            </div>

          </form>
        </div>
      </div>

    }
}


#[component]
fn ResetPasswordPage(cx: Scope) -> impl IntoView {

    view! { cx,

        <div class="flex items-center justify-center h-screen bg-gray-100">
        <div class="w-80 bg-white rounded shadow p-6">
          <h2 class="text-2xl font-semibold mb-6">"Reset Password"</h2>
          <form>
            <div class="mb-4">
              <label for="email" class="block text-gray-700 font-semibold mb-2">"Email"</label>
              <input id="email" type="email" name="email" placeholder="Enter your email"
                class="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:border-indigo-500"></input>
            </div>
            <div class="flex items-center justify-between">
              <button type="submit"
                class="bg-indigo-500 hover:bg-indigo-600 text-white font-semibold py-2 px-4 rounded focus:outline-none focus:shadow-outline">
                "Reset Password"
              </button>
              <a href="/" class="text-gray-600 hover:text-indigo-500">"Back to Sign In"</a>
            </div>
          </form>
        </div>
      </div>

    }
}
