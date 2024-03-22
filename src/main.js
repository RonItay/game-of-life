const { invoke } = window.__TAURI__.tauri;
const { emit, listen } = window.__TAURI__.event;

import { GRID_SIZE, COLORS } from "./consts.js";

// events sent to backend

/* Notify that the user pressed a square */
async function notify(pressed_id) {
  let coordinates = pressed_id.split("-");
  await emit("notify", {row: parseInt(coordinates[0]), col: parseInt(coordinates[1])});
}

/* Start running a continous calculation */
async function run() {
  await emit("run", {});
}

/* Start running a continous calculation */
async function stop() {
  await emit("stop", {});

}


// Commands to the backend

/* Clear the board */
async function clear() {
  let resulting_squares = await invoke("clear", {});
  resulting_squares = JSON.parse(resulting_squares);
  toggle_squares(resulting_squares);
}

/* Calculate one step */
async function step() {
  let resulting_squares = (await invoke("step", {}));
  resulting_squares = JSON.parse(resulting_squares);
  toggle_squares(resulting_squares);
}

async function select_object(choice) {
  await clear();
  let resulting_squares = JSON.parse(await invoke("select", {object: choice}));
  toggle_squares(resulting_squares);
}

/* Events from the backend */
listen("update", (event) => {
  toggle_squares(event.payload);
});



// Utility functions

function toggleColor(element) {
  for (let color of COLORS) {
    element.classList.toggle(color);
  }
}

function toggle_squares(squares) {
  for (let changed_square of squares) {
    let row = changed_square["row"];
    let col = changed_square["col"];

    // make sure the square is in the screen
    if (row < 0 || row >= GRID_SIZE) {continue};
    if (col < 0 || col >= GRID_SIZE) {continue};

    let curr_square = document.getElementById(row.toString() + "-" + col.toString());
    toggleColor(curr_square)
  }
}
 
export function onClick(event) {
  var square = event.target;
  toggleColor(square);
  notify(square.id);
}


// bind buttons to events

window.addEventListener("DOMContentLoaded", () => {
  document.querySelector("#stepButton").addEventListener("click", (e) => {
    step();
  });
});

window.addEventListener("DOMContentLoaded", () => {
  document.querySelector("#clearButton").addEventListener("click", (e) => {
    clear();
  });
});

window.addEventListener("DOMContentLoaded", () => {
  document.querySelector("#runButton").addEventListener("click", (e) => {
    console.log(e);
    let target = e.target;
    let currently_active = target.classList.contains("active");
    if (currently_active) {
      stop();
    } else {
      run();
    }
    target.classList.toggle("active");
  });
});


window.addEventListener("DOMContentLoaded", () => {
  document.querySelector("#selectObject").addEventListener("change", (e) => {
    var selectedOption = document.getElementById('selectObject').value;
    select_object(selectedOption)
  });
});

