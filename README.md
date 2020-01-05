# Smart Falcon

This is home to _Smart Falcon_, a simple minimalistic web application project that displays the odds that Millenium Falcon reaches the Rebel forces before the Empire.

## Getting Started

### Building
1. You need to have rust and Cargo installed to build the project:
```bash
curl https://sh.rustup.rs -sSf | sh
```
2. The biggest dependency of this project is Rocket. As a consequence you need to use the nightly toolchain to build:
```bash
rustup toolchain install nightly
rustup default nightly
```
3. Then to build and run the server you just need to invoke cargo run (with the production Rocket configuration variable) as follows:
```bash
 ROCKET_ENV=production cargo run
```

### Architecture

- The `/data` folder contains the input json data files and the `universe.db` sqlite db file.
- The front-end corresponds to a single web page. It is contained in the `/static` folder and served by Rocket on the main route `/`. All the routes are in the `/src/c_3po` folder.
- The back-end is inside the `/src/millenium_falcon_onboard_computer` folder. It's mainly managed by the Rocket framework. I chose Rocket because it offers a central point to manage front and back in a simple way. It takes in charge the routing, the pre-processing and the post-processing of user queries.

## Algorithmic solution

To compute the odds for the Falcon to save Endor, I propose a two-steps approach. During the first step I generate all possible paths from departure to arrival subject to autonomy and countdown constraints. During the second step I compute the odds for each valid path and I select the best one.
Finding the "best path" from departure planet to source planet cannot be done with "standard" algorithms (DFS, BFS, A*, Bellmann...) because of the autonomy constraints. I implemented a variant of DFS with "backtrack": when countdown is reached or autonomy null the Falcon backtracks.
