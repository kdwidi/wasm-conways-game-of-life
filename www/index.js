import { Universe } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

const CELL_SIZE = 7;
const GRID_COLOR = "#E0E0E0";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";
const FONT_SIZE = CELL_SIZE * 0.6;

const universe = Universe.new(192, 96);
const width = universe.width();
const height = universe.height();
let animationId = null;

const fps = new (class {
  constructor() {
    this.fps = document.getElementById("fps");
    this.frames = [];
    this.lastFrameTimestamp = performance.now();
  }

  render() {
    const now = performance.now();
    const delta = now - this.lastFrameTimestamp;
    this.lastFrameTimestamp = now;
    const fps = (1 / delta) * 1000;

    this.frames.push(fps);
    if (this.frames.length > 100) {
      this.frames.shift();
    }

    let min = Infinity;
    let max = -Infinity;
    let sum = 0;
    for (let i = 0; i < this.frames.length; i++) {
      sum += this.frames[i];
      min = Math.min(this.frames[i], min);
      max = Math.max(this.frames[i], max);
    }
    let mean = sum / this.frames.length;

    this.fps.textContent = `fps: ${Math.round(
      fps
    )} | avg of last 100s: ${Math.round(mean)}
    `.trim();
  }
})();

// canvas
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

canvas.addEventListener("click", (e) => {
  const boundingRect = canvas.getBoundingClientRect();

  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;

  const canvasLeft = (e.clientX - boundingRect.left) * scaleX;
  const canvasTop = (e.clientY - boundingRect.top) * scaleY;

  const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
  const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

  universe.toggle_cell(row, col);

  drawCells();
});

const ctx = canvas.getContext("2d");

// button
const playPauseButton = document.getElementById("play-pause");

const play = async () => {
  playPauseButton.textContent = "PAUSE";
  await renderLoop();
};

const render = async (resolve) => {
  return new Promise(() => {
    fps.render();
    universe.tick();
    drawCells();
    if (resolve) resolve();
  });
};

const pause = () => {
  playPauseButton.textContent = "PLAY";
  cancelRenderAnimation();
};

playPauseButton.addEventListener("click", () => {
  if (animationId === null) {
    play();
    console.log("played");
  } else {
    pause();
    console.log("paused");
  }
});

// clear
const clearButton = document.getElementById("clear");
clearButton.addEventListener("click", () => {
  console.log("reset clicked");
  universe.clear_cells();
  drawCells();
  pause();
});

// random
const random = document.getElementById("random");
random.addEventListener("click", () => {
  universe.set_cells(generateRandomCells());
  drawCells();
  pause();
});

const generateRandomCells = () => {
  let cells = [];
  for (let i = 0; i < width * height; i++) {
    let rand = Math.random();
    if (rand < 0.3) cells.push(i);
  }
  return cells;
};

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }

  for (let i = 0; i <= height; i++) {
    ctx.moveTo(0, i * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, i * (CELL_SIZE + 1) + 1);
  }
  ctx.stroke();
};

const getIndex = (row, col) => {
  return row * width + col;
};

const bitIsSet = (n, arr) => {
  const byte = Math.floor(n / 8);
  const mask = 1 << n % 8;
  return (arr[byte] & mask) === mask;
};

const drawCells = () => {
  const cells = new Uint8Array(
    memory.buffer,
    universe.cells(),
    (width * height) / 8
  );

  ctx.beginPath();

  ctx.fillStyle = ALIVE_COLOR;
  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const index = getIndex(row, col);
      if (!bitIsSet(index, cells)) continue;

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }
  ctx.fillStyle = DEAD_COLOR;
  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const index = getIndex(row, col);
      if (bitIsSet(index, cells)) continue;

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }
  ctx.stroke();
};

const drawIndex = (x, y, index) => {
  ctx.beginPath();
  ctx.font = "bold " + FONT_SIZE + "px Arial";
  ctx.fillStyle = "white";
  ctx.textAlign = "center";
  ctx.fillText(index + "", x + CELL_SIZE * 0.5, y + CELL_SIZE * 0.8);
  ctx.stroke();
};

const renderLoop = async () => {
  await render(() => {
    setTimeout(() => {
      animationId = requestAnimationFrame(renderLoop);
    }, 0);
  });
};

const cancelRenderAnimation = () => {
  cancelAnimationFrame(animationId);
  animationId = null;
};

drawGrid();
