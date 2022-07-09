use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <div id="home">
            <header>
                <h1>{ "Rust Algorithms" }</h1>

                <p>{ "Visualize various types of algorithms in a responsive, interactive and configurable way." }</p>
            </header>

            <section class="about">
                <h2>{ "About" }</h2>

                <h3>{ "How is this project made?" }</h3>

                <p>
                    { "Everything in this project is made entirely by me in Rust with the " } <a href="https://yew.rs" target="_blank">{ "Yew web framework" }</a> { ". If you want, you can view the code " } <a href="https://github.com/Jondolf/rust-algorithms" target="_blank">{ "here" }</a> { ". I am not a Rust, Yew or algorithm expert however (I'm in high school), so many things may not necessarily be optimal or compliant with best practises."
                    }
                </p>

                <h3>{ "Why am I making this?" }</h3>

                <p>{ "TLDR: Learning and fun." }</p>

                <p>
                    {
                        "I have always wanted to learn about different kinds of technologies and algorithms. Inspired by some "
                    }
                    <a href="https://www.youtube.com/watch?v=kPRA0W1kECg" target="_blank">{ "sorting algorithm videos" }</a>
                    {
                        ", I ended up implementing a similar sorting visualizer as it seemed relatively straightforward and fun to make. Around that time I also found the Yew framework, and decided that it would be the perfect match as I wanted to use Rust in the project anyway."
                    }
                </p>

                <p>
                    {
                        "After finishing up most of the sorting visualizer, I started exploring other algorithm types. I ended up making a pathfinding visualizer as well, as I find it to be a really fun, interesting and useful learning experience. In the future I'll keep adding more features and algorithms when I find the time and interest to do so."
                    }
                </p>

                <h3>{ "Future" }</h3>

                <p>
                    {
                        "I don't really have any concrete plans that I will stick to, as this is a hobby project and I will usually be busy with school when I'm not on vacation."
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
                    <li>{ "More algorithm descriptions" }</li>
                    <li>{ "Render algorithm code on the page" }</li>
                    <li>{ "Website themes" }</li>
                </ul>
            </section>
        </div>
    }
}
