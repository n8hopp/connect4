# connect4

Connect4 game written in Rust for CS352: Programming Languages

This game can be played against a friend on the same computer (or against yourself, if you're feeling lonely, but I would strongly advise against that)

Prompts user for column they wish to play in. If a player enters an invalid move 5 times, the game auto-plays for them. I suppose you could emulate a pretty basic randomized CPU if you just forced this function to run for one player's turn.

I had a lot of fun making this, I actually enjoy Rust a lot. Please excuse if there are any places I majorly butchered Rust syntax, this was my first time writing in the language.

I did deviate from some originally-defined function definitions in the assignment, because I believed it made the flow of the program easier to understand. For example, here's a change that I made:
* human_turn was defined to "return if there was a winner" -> human_turn returns upon a successful piece place

I was not keeping track of all of these types of changes, so to the grader, apologies if they're in greater number than was anticipated/accounted for.
