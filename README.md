# Untitled Tank RPG
Top-down competitive multiplayer game made in Rust using Bevy

## Current usage
Download and boot up the client executable from our releases to play!

The client is capable of joining servers and hosting servers. If you would like to host a server
without joining as a client, you can use our server executable. You may pass the `headless` arg to
the server executable to run a lightweight headless version as well. 
Currently the server executable defaults to hosting on port `1337`.

As the client you may host on any port that you'd like, and join servers by entering their address
(e.g. `127.0.0.1:1337`)

Very basic combat is implemented. You can shoot bullets at other connected players on the server and
that's about it (so far). Player's have their health displayed, there is a leaderboard noting kill
counts for each player, and players respawn at the least crowded spawn location after 5 seconds.

Next steps are to include a match timer, so matches can be finished with a final leaderboard,
and implementing a simple map with collision and obstacles.

Further in the future, the player will be able to equip skills dropped from other players, and
level up based on kills. More complex movement and mechanics will be added such as dashing and 
bullet bouncing.