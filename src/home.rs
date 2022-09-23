use crate::icon::Icon;
use crate::navbar::{Navbar, NavbarLink};
use crate::repository_card::RepositoryCard;
use crate::section::Section;
use crate::timeline::{Timeline, TimelineEvent};
use crate::typography::{Heading, Link, Paragraph};
use js_sys::Date;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    return html! {
        <main class="min-h-screen bg-gray-2">
            <HomeIntroductionSection />
            <HomeProjectsSection />
            <HomeExperienceSection />
            <HomeContactSection />
        </main>
    };
}

#[function_component(HomeIntroductionSection)]
fn home_introduction_section() -> Html {
    return html! {
        <Section class="bg-gray-2" id="#top">
            <Navbar>
                <NavbarLink href="#projects">{"Projects"}</NavbarLink>
                <NavbarLink href="#experience">{"Experience"}</NavbarLink>
                <NavbarLink href="#contact">{"Contact"}</NavbarLink>
            </Navbar>

            <div class="w-full flex flex-col md:flex-row pt-4 gap-2 md:py-8">
                <div class="w-full md:w-1/3 px-3">
                    <div class="w-full h-full">
                        <img alt="profile picture" class="rounded-full" src="/picture.webp" width={300} height={300} />
                    </div>
                </div>

                <div class="w-full md:w-2/3 px-3 flex flex-col justify-center">
                    <div>
                        <Heading primary={true}>{"👋Hi, I'm Mats!"}</Heading>
                        <Paragraph>{"I'm a developer and student based in Trondheim, Norway with a passion for building things that people love. I love to explore new things, work on free open-source software, and cook food."}</Paragraph>
                        <Paragraph>{"I'm currently enrolled in a bachelor's degree in informatics at NTNU in Trondheim, Norway. On the side I'm remotely working part-time as a frontend developer and working on a number of open-source projects on GitHub."}</Paragraph>
                    </div>
                </div>
            </div>
        </Section>
    };
}

#[function_component(HomeProjectsSection)]
fn home_projects_section() -> Html {
    return html! {
        <Section class="bg-gray-1" id="projects">
            <div class="p-3">
                <Heading>{"🔨Open Source"}</Heading>
                <Paragraph>{"I'm a firm believer in open-source software, and I consequently find myself spending a lot of time working on free, open-source projects on GitHub. Some of the projects I work on were created by me, most by others. Here are some of my highlights."}</Paragraph>
                <div class="grid md:grid-cols-2 gap-2 mt-4">
                    <RepositoryCard
                        org="compiler-explorer"
                        repo="compiler-explorer"
                        stack={vec!["typescript".to_owned(), "nodejs".to_owned()]}
                        contribution="I've worked across the entire Compiler Explorer stack, but most of my work has been on transitioning from JavaScript to TypeScript. I've also done extensive work on the code formatters and assembly documentation hints in the editor. I'm also responsible for some the Rust tooling on the site."
                        description="Compiler Explorer, or commonly known as godbolt.org, is an interactive website where you can use over 1500 compilers and inspect their assembly outputs in the browser."
                    />
                    <RepositoryCard
                        org="matsjla"
                        repo="league-connect"
                        stack={vec!["typescript".to_owned(), "nodejs".to_owned()]}
                        contribution="I started development on league-connect in 2019. After many iterations and a number of releases following semantic versioning it has become the most popular Node.js library for interacting with the League Client."
                        description="League Connect is my first open-source Node.js library published to NPM that enables desktop applications and tools to access the League of Legends Client APIs. It works over a mix of HTTP/1.1, HTTP/2.0 and WebSockets."
                    />
                    <RepositoryCard
                        org="llvm"
                        repo="llvm-project"
                        stack={vec!["cplusplus".to_owned(), "c".to_owned(), "cmake".to_owned()]}
                        contribution="My work on LLVM has mostly been within LLVM's C interface, a way to access LLVM APIs from the C programming language. Here I've worked on mapping functions and adding tests related to the LLVM IR and LLVM OrcJITv2 runtime compiler."
                        description="LLVM is a monolithic compiler infrastructure project hosting a number of C and C++ compilers, optimizers, and standard library implementations. It's also the compiler backend for a significant amount of programming languages such as C, C++, Rust, and Swift"
                    />
                    <RepositoryCard
                        org="dotkom"
                        repo="monoweb"
                        stack={vec!["typescript".to_owned(), "react".to_owned(), "nextjs".to_owned(), "aws".to_owned(), "terraform".to_owned()]}
                        contribution="During my time as an Online and Dotkom member I've contributed to numerous parts of our system including our design system, our cloud infrastructure hosted on Amazon Web Services, and most recently; our new events system."
                        description="Monoweb is the new iteration of my study programme's student association's website. It's a large system with many components varying in complexity. Monoweb is implemented as full-stack TypeScript and is the successor to our legacy Python system."
                    />
                </div>
            </div>
        </Section>
    };
}

#[function_component(HomeExperienceSection)]
fn home_experience_section() -> Html {
    let events: Vec<(String, TimelineEvent)> = vec![(
        "Consigli AS".to_owned(),
        TimelineEvent::new(
            Date::new_with_year_month(2021, 11),
            Date::new_0(),
            "Frontend Developer (part-time, remote)".to_owned(),
            vec![
                "Developer on Consigli platform frontend team, building a web platform using TypeScript and React & Redux with a focus on code architecture and Atomic Design.".to_owned(),
                "DevOps engineer on Microsoft Azure with GitHub Actions, enabling continuous deployment of web platform to staging and production environments using Bicep code with Azure Container Registry.".to_owned()
            ],
            vec!["typescript".to_owned(), "react".to_owned(), "azure".to_owned()],
        ).active(),
        ),(
        "Nelfo".to_owned(),
        TimelineEvent::new(
            Date::new_with_year_month(2019, 7),
            Date::new_with_year_month(2021, 7),
            "Frontend Developer (part-time)".to_owned(),
            vec![
                "Led planning, development, and design for static web content platform used to promote electrical engineering to norwegian high school students.".to_owned(),
                "Was responsible for deployment to production environment to static file host over FTP with nightly builds deploying to Vercel.".to_owned()
            ],
            vec!["typescript".to_owned(), "react".to_owned(), "nextjs".to_owned()],
        ),
    )];

    return html! {
        <Section class="bg-gray-2" id="experience">
            <div class="p-3">
                <Heading>{"💼Experience"}</Heading>
                <Paragraph>{"Throughout the years I've been lucky enough to work for a few companies where I've grown a lot as a developer, as well has having the opportunity to work in teams on larger scale software. This is a timeline highlighting my work at these companies and what roles I've taken."}</Paragraph>

                <div class="my-4">
                    <Timeline events={events} />
                </div>
            </div>
        </Section>
    };
}

#[function_component(HomeContactSection)]
fn home_contact_section() -> Html {
    return html! {
        <Section class="bg-gray-1" id="contact">
            <div class="p-3">
                <Heading>{"✉️Contact"}</Heading>
                <Paragraph>{"I'm always interesting in expanding my network and building connections. Have any questions, thoughts, or ideas? Or maybe an opportunity you think I'd be interested in? Please reach me through one of these links, I'm always up for a chat!"}</Paragraph>

                <ul class="my-4">
                    <li>
                        <Paragraph>
                            <Icon name="linkedin" alt="linkedin" />
                            {"Connect with me on "}
                            <Link href="https://linkedin.com/in/mats-jun-larsen" external={true}>{"LinkedIn"}</Link>
                        </Paragraph>
                    </li>
                    <li>
                        <Paragraph>
                            <Icon name="github" alt="github" />
                            {"Follow my activity on "}
                            <Link href="https://github.com/matsjla" external={true}>{"GitHub"}</Link>
                        </Paragraph>
                    </li>
                    <li>
                        <Paragraph>
                            <Icon name="gmail" alt="email" />
                            {"Shoot me an "}
                            <Link href="mailto:me@supergrecko.com" external={true}>{"email"}</Link>
                        </Paragraph>
                    </li>
                </ul>

                <div class="text-center font-bold mt-6">
                    <Paragraph>
                        {"Built using "}
                        <Icon name="rust" alt="rust" />
                        {" & "}
                        <Icon name="wasm" alt="webassembly" />
                        {" with <3 — © 2022"}
                    </Paragraph>
                </div>
            </div>
        </Section>
    };
}
