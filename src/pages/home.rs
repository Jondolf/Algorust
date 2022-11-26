use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component]
pub fn HomePage() -> Html {
    html! {
        <div id="home">
            <header>
                <h1>{ "Algorust" }</h1>

                <p>{ "Visualize various types of algorithms in a responsive, interactive and configurable way. Everything is made with Rust 🦀" }</p>

                <div class="buttons">
                    <Link<Route> to={Route::Sorting}>
                        <button class="sorting-button">{ "Go to sorting" }</button>
                    </Link<Route>>

                    <a href="https://github.com/Jondolf/rust-algorithms" target="_blank" aria-label="Link to this website's GitHub repository (opens in a new window)">
                        <button class="github-button">
                            <img
                                src="/assets/images/GitHub-Mark-Light-64px.png"
                                alt="GitHub logo"
                                width="20"
                                height="20"
                            />
                            { "View the code" }
                        </button>
                    </a>
                </div>
            </header>

            <section class="about">
                <h2>{ "About" }</h2>

                <h3>{ "How is this project made?" }</h3>

                <p>
                    {
                        "Practically everything in this project is made entirely by me in Rust with the " } <a href="https://yew.rs" target="_blank">{ "Yew web framework" }</a> { ". Only the necessities such as config files, the root " } <code>{ "index.html" }</code> { " file and the SCSS styles are not written in Rust, as it's not currently feasibly possible to do so."
                    }
                </p>

                <p>
                    {
                        "If you want, you can view the code at the project's " } <a href="https://github.com/Jondolf/rust-algorithms" target="_blank">{ "GitHub repository" }</a> { ". I am not a Rust, Yew or algorithm expert however (I'm in high school), so many things may not necessarily be optimal or compliant with best practises."
                    }
                </p>

                <h3>{ "Why am I making this?" }</h3>

                <p>{ "TLDR: Learning and fun." }</p>

                <p>
                    {
                        "While I was learning Rust, I stumbled upon the Yew web framework. I was interested in building a web application entirely in Rust, and decided that a website focused on algorithms would be perfect for both learning and testing out Rust's performance."
                    }
                </p>

                <p>
                    {
                        "I ended up making a sorting visualization website with many different algorithms and configuration options. This was a great learning experience, but I wanted to look into other types of algorithms as well. I broadened the scope of the website from just sorting algorithms to algorithms in general, and added a pathfinding visualizer."
                    }
                </p>

                <p>{ "In the future I'll keep adding more features and algorithms when I find the time and interest to do so." }</p>

                <h3>{ "Future" }</h3>

                <p>
                    {
                        "I don't really have any concrete plans that I will stick to, as this is a hobby project and I will usually be busy with school outside of vacations."
                    }
                </p>

                <p>{ "Some potential features include:" }</p>

                <ul>
                    <li>
                        { "More different types of algorithms, such as" }
                        <ul>
                            <li>{ "Search algorithms (binary search etc.)" }</li>
                            <li>{ "Shuffle algorithms (maybe integrated into sorting, so you can shuffle the input and then sort it)" }</li>
                            <li>{ "Maybe simple cryptography? (encrypt/decrypt messages step-by-step)" }</li>
                        </ul>
                    </li>
                    <li>
                        { "More features and configuration options to existing algorithm types" }
                        <ul>
                            <li>{ "Sorting: value affects color, a list of strings as input, textual input and output" }</li>
                            <li>{ "Pathfinding: weight maps" }</li>
                        </ul>
                    </li>
                    <li>{ "Render algorithm code implementations on the page (at least pseudocode and Rust, maybe others as well)" }</li>
                </ul>
            </section>
        </div>
    }
}
