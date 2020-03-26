import { GameState } from "game-2048";

const game = GameState.new();
var moves = 0;

const pre_moves = document.getElementById("moves");
const pre = document.getElementById("game");
pre.textContent = game.render();
pre_moves.textContent = "moves: " + moves + "\t\t\t\tPress ↑ ← ↓ → to play";

document.onkeydown = function (event) {
    switch (event.keyCode) {
        case 37:
            game.left(true);
            break;
        case 38:
            game.up(true);
            break;
        case 39:
            game.right(true);
            break;
        case 40:
            game.down(true);
            break;
    }
    var game_over = game.check_game_over();
    if (game_over) {
        pre.textContent = "\t\tGAME OVER\n" + game.render();
    } else {
        moves += 1;
        pre.textContent = game.render();
    }

    pre_moves.textContent = "moves: " + moves + "\t\t\t\tPress ↑ ← ↓ → to play";
};

function check_game_over() {
}
