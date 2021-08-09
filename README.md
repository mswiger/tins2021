# TINS 2021 - I Want to Go Home

Navigate through a randomly generated island and try to find the portal to go back home. 
Move by clicking on tiles that are adjacent to your character.
The camera can be controlled with WASD or the arrow keys.

## TINS Rules
* **genre rule #143 - Humoristic/Funny:** I tried to do a few things to make the player laugh (title card, soundtrack).
* **artistical rule #147 - Inspired by MC Escher:** The game's tilemap is a tessellated "grid" of hexagons.
* **artistical rule #94 - The game should contain a plug:** I plug my other barely-a-game in the title card.
* **technical rule #113 - Something in the game must be hexagonal:** The tiles are all hexagons.
* **bonus rule #13 - Test of Might (add unit tests)**: I added a single unit test that can be executed with `cargo test`. If I did too poorly on one of the above rules, swap this one in for that one :).

## Known Bugs
Currently on Windows, the cursor coordinates are way off.
If you are having trouble moving the character, try moving the mouse around the top-right-ish part of the screen until you see a yellow hexagon cursor appear near your character.
That hexagon represents where the game thinks your cursor is. You can kind of make it work by clicking around a bunch when you find that spot on the screen.

## Links
https://tins.amarillion.org/2021/
