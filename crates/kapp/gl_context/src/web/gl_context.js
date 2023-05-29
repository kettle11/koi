// This file is unused at the moment.

function create_webgl1_context(is_display_p3) {
  let canvas = document
    .getElementById("canvas");

  // There are other potentially useful flags as well.
  let gl = canvas.getContext('webgl', {
    alpha: false,
    desynchronized: false,
    antialias: true,
    depth: true
  });
}

function create_webgl2_context(is_display_p3) {
  console.log("CREATING WEBGL2 CONTEXT");

  let canvas = document
    .getElementById("canvas");

  // There are other potentially useful flags as well.
  canvas.getContext('webgl2', {
    alpha: false,
    desynchronized: false,
    antialias: true,
    depth: true,
  });
}

function command(command, is_display_p3) {
  switch (command) {
    case 0:
      create_webgl1_context(is_display_p3);
      break;
    case 1:
      create_webgl2_context(is_display_p3);
      break;
    case 2:
      document
        .getElementById("canvas");
  }
  return 0;
}
command
