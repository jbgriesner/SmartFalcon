# what_are_the_odds

This is home to _what_are_the_odds_, a simple minimalistic web application project that displays the odds that Millenium Falcon reaches the Rebel forces before the Empire.
The solution is deployed on my personal server.
You can test my implementation online on griesner.net:8000

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

### Architecture and Assumptions

The `/data` folder contains the input json data files and th `universe.db` sqlite db file.
The front-end corresponds to a single web page. It is contained in the `static` folder and served by Rocket on the main route `/ `. All the routes are in the `c_3po` folder.
The back-end is inside the `/src/millenium_falcon_onboard_computer` folder.

I have made the following assumptions to limit data validation and thus code faster:
- `data/universe.db` well formed

## Algorithmic solution

To compute the odds for the Falcon to save Endor, I propose a two-steps
Finding the "best path" from departure planet to source planet cannot be done with "standard" algorithms (DFS, BFS, A*, Bellmann...) because of the autonomy constraints.
1/ Generate all paths

why DFS ?
allows to make the computations once and then everything is ok for the uploading file

Complexity analysis: enormous!

## Why Rocket & Technical choices

Rocket framework offers a central point to manage front and back in a simple way.
It takes in charge the routing, the pre-processing and the post-processing of user queries.

## Personal notes

From a personal point of view I really enjoyed to work on this project for the following reasons:
-
-
- it was the first time I used Rocket. I was only used to iron, actix...

Of course my implementation leaves room for many improvements.
Optimization
- I think tests are 100% necessary
