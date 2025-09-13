# HackPortfolio

Hackable Portfolio Template written in Rust with Dioxus

See the demo on [hack-portfolio.reify.ing](https://hack-portfolio.reify.ing)!

Also check out the story behind this project - [Rewrite It in Rust with AI - Oxidize a Portfolio Website in a Day](https://ideas.reify.ing/en/blog/rwiir-with-ai/)

## Usage

1. Clone the repository
2. Make sure you have Rust and
   `dioxus-cli` [installed](https://dioxuslabs.com/learn/0.6/getting_started/#install-the-dioxus-cli).
3. Run this project with hot reloading:
    ```shell
   dx serve
   ```
4. Build the project:
    ```shell
   dx bundle --platform web --ssg
   ```
   You will see the bundled files in `target/dx/hack-portfolio/release/web/public`.

## How to hack / customize

Fork the repo and:

1. If you want to just change the content, modify [`src/personal_info/mod.rs`](src/personal_info/mod.rs) and add your
   images to [`src/personal_info/images`](src/personal_info/images).
2. If you want to add more data to sections, take a look at [`src/data.rs`](src/data.rs) and modify the data types.
3. You can modify the styles by editing the CSS files in each UI component's directory. For
   example, [the CSS for project cards](src/components/github_repo_card/github_repo_card.css).
4. Moreover, you can modify the layout of components and pages by editing the `.rs` files in each component's directory.

Happy hacking!

## Vercel Deployment

1. Fork this repo
2. Import your fork in Vercel, creating a Vercel project
3. In the project settings - Build and Deployment, override the following
   * Build Command -> `bash vercel-build.sh`
   * Install Command -> `bash vercel-install.sh`
   * Output Directory -> `public`

## TODOs

* [ ] Fix font loading
* [ ] UI polishments

## License

[MIT](LICENSE)

## Acknowledgements

This project is based on [masterPortfolio](https://github.com/ashutosh1919/masterPortfolio) which is licensed under MIT
license.