
const HEIGHT = 800
const WIDTH = 800

const ROT_X = 0.8
const ROT_Y = 0.3
const ROT_Z = 0
const ROT_SPEED = 0.01

function setup() {
  createCanvas(HEIGHT, HEIGHT, WEBGL);
  boxes = []
  b = new Box(0, 0, 0, HEIGHT/4, WIDTH/4, HEIGHT/4)
  boxes.push(b)
  a = 0.01
  rotation_vec = createVector(ROT_X, ROT_Y, ROT_Z)
}

function draw() {
  background(255);
  //stroke(255)

  lights()

  rotate(a, rotation_vec)
  for (let i = 0; i < boxes.length; i++) {
    boxes[i].show()
  }

  a  += ROT_SPEED
}

function mousePressed() {
  let next = []
  for (let i = 0; i < boxes.length; i++) {
    let newBoxes = boxes[i].generate()
    next = next.concat(newBoxes)
  }
  boxes = next
}