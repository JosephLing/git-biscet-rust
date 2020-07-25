### assignment
https://rgrig.github.io/plad/homework.html

## Solution:
- The core the of the algorithm can be found in `algorithm.rs` which contains the good and bad commit removal. Along with the get_next_commit function which is used to pick which commit to ask a question about.
- Generally it is pretty fast general case solution although not optimised too well for really large repositories. Yet has only given up on 165 to 199 instances throughout the 10,000 instances which is pretty good with an average of 8-9 questions being asked.
  - It normally runs in just over 3hrs but the string to integer suggestion you made Radu on the forum sped it up by around 30 minutes on the submission. It is in `integer-rewrite` branch but some of the unit tests failed as they were expecting strings and I had other things to do than rewrite the unit tests (git tree is to large to zip up as the test json files are in the commit history.... the irony).
  - I had given it a go and heard that randomly picking after a size of graph might work a bit better but without a more hueristic approach I wasn't able to get a better score doing that.

## Testing:
- unit tests in algorithms and json_types
  - These are used to make sure that individual parts of the implementation. The key aspect was that they allowed for development of small parts of the system without having to implement the websocket client to be able to test whether or not it worked.
- note: as rust tests run in parallel and in order to do the integeration tests I needed to have a webserver up for each it will create a hundred or so websockets in port range 3000+ on your computer if you run `cargo test`. This does mean that the tests take about a minute or two too run though which is really nice! :) 
- integeration tests in 'tests':
  - This was really interesting and nice find in how Rust seperates tests in that it will run all the inline unit tests. Then after that it will run the integeration tests which are found in the tests folder.
  - These were created to allow offline development primarily without having to use the vpn or ssh into raptor. As well as near submission time the test server could have been slow.
  - Additionally I wanted a way to be able to test the how the solution handled different situations. But have the ability to run against a single test that checked one thing until I fixed that one thing. Therefore allowing for easier debugging and development.
  - The integeration tests are in 3 parts:
    - `integeration_tests.rs` -> these are my first set of tests testing out the client to server interactions
    - `integeration_large_tests.rs` -> after I had got the previous ones working I implemented the offline json files to load up in the same way (note: for the tests I didn't care too much about the best code quality so there is a lot of cloning and similar code between all the integeration suites)
    - `tiny_server_integeration_tests.rs` -> I had mucked up in creating the `server.rs` and had it responding with "bad" instead of "Bad" which meant that it passed all the tests locally but failed for all of them for the test server. In order to try and fix this I pulled tests from the test server using: https://github.com/JosephLing/gitbisectscaffolding-scrapper and in order to triple check my implementation. However the spec I had implemented in the client and server was wrong by one character.....
      - the .dot file creation with https://www.graphviz.org/ helped debug this situation although it was all a bit of a mess... 
    - `server.rs` -> a this server was able to run a repo with multiple instances (and potentailly multiple repos... never tested it). The key aspects of this was that it failed if the solution was wrong and if the client gave up or didn't respond quickly enough with the server.

