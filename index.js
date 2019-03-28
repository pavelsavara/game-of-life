var canvas = document.getElementById("canvas");
var displayWidth = canvas.clientWidth;
var displayHeight = canvas.clientHeight;

// Check if the canvas is not the same size.
if (canvas.width != displayWidth ||
  canvas.height != displayHeight) {

  // Make the canvas the same size
  canvas.width = displayWidth;
  canvas.height = displayHeight;
}

import('./pkg/game_of_life')
  .then(gol => {
    gol.render(11.0);
  })
  .catch(console.error);

