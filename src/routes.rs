use crate::{
    blog::blog_page::BlogPage,
    home::{home_hero::HomeHero, home_page::HomePage},
    projects::projects_page::ProjectsPage,
    resume::resume_page::ResumePage,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn MainRoutes() -> impl IntoView {
    view! {
        <Stylesheet id="leptos" href="/pkg/chinxeleer-rust.css" />

        // sets the document title
        <Title text="chinxeleer" />

        // content for this welcome page
        <main class="bg-slate-900 text-purple-200 min-h-screen ">
        <div class="max-w-5xl mx-auto ">
        <Routes>
             <Route path="" view=HomePage >
                    <Route path="" view=HomeHero />
                    <Route path="resume" view=ResumePage />
                    <Route path="projects" view=ProjectsPage />
                    <Route path="blog" view=BlogPage />
                </Route>

            </Routes>
        </div>

        </main>
    }
}
