
class Box {
    pos;
    r;

    constructor(x, y, z, r) {
        this.pos = createVector(x, y, z);
        this.r = r;
    }

    //Subdivide the box using the Menger Sponge algorithm
    generate() {
        let newBoxes = [];
        let newR = this.r / 3;
        for (let x = -1; x < 2; x++) {
            for (let y = -1; y < 2; y++) {
                for (let z = -1; z < 2; z++) {
                    let sum = abs(x) + abs(y) + abs(z);
                    let newP = new createVector(this.pos.x + x * newR, this.pos.y + y * newR, this.pos.z + z * newR);
                    if (sum > 1) {
                        let b = new Box(newP.x, newP.y, newP.z, newR);
                        newBoxes.push(b);
                    }
                }
            }
        }
        return newBoxes;
    }

    show() {
        push();
        translate(this.pos.x, this.pos.y, this.pos.z);

        fill(220);
        box(this.r);
        pop();
    }

}