export class Vector3 {

    constructor(public x: number, public y: number, public z: number) {

    }

    into() {
        return [this.x, this.y, this.z];
    }
}