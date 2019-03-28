var canvas = document.getElementById("canvas");
var display = Math.min(canvas.clientWidth,canvas.clientHeight);

// Check if the canvas is not the same size.
if (canvas.width != display ||
  canvas.height != display) {

  // Make the canvas the same size
  canvas.width = display;
  canvas.height = display;
}

import('./pkg/game_of_life')
  .then(gol => {
    gol.new_board();
    setInterval(() => {
      gol.render();
    }, 0)
  })
  .catch(console.error);

