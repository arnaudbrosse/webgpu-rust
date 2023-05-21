mod math;
mod core;

use math::color::Color;
use math::matrix4::Matrix4;
use math::quaternion::Quaternion;
use math::vector3::Vector3;

fn main() {
    let vec1 = Vector3::new(1.0, 2.0, 3.0);
    let vec2 = Vector3::new(4.0, 5.0, 6.0);

    let sum = vec1 + vec2;
    println!("Sum: {:?}", sum);

    let diff = vec2 - vec1;
    println!("Difference: {:?}", diff);

    let scaled = vec1 * 2.0;
    println!("Scaled: {:?}", scaled);

    let dot_product = vec1.x * vec2.x + vec1.y * vec2.y + vec1.z * vec2.z;
    println!("Dot Product: {}", dot_product);

    let cross_product = Vector3::new(
        vec1.y * vec2.z - vec1.z * vec2.y,
        vec1.z * vec2.x - vec1.x * vec2.z,
        vec1.x * vec2.y - vec1.y * vec2.x,
    );
    println!("Cross Product: {:?}", cross_product);

    let mat1 = Matrix4::new([
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 10.0, 11.0, 12.0],
        [13.0, 14.0, 15.0, 16.0],
    ]);

    let mat2 = Matrix4::new([
        [17.0, 18.0, 19.0, 20.0],
        [21.0, 22.0, 23.0, 24.0],
        [25.0, 26.0, 27.0, 28.0],
        [29.0, 30.0, 31.0, 32.0],
    ]);

    let sum = mat1 + mat2;
    println!("Sum: {:?}", sum);

    let diff = mat2 - mat1;
    println!("Difference: {:?}", diff);

    let scaled = mat1 * 2.0;
    println!("Scaled: {:?}", scaled);

    let product = mat1 * mat2;
    println!("Product: {:?}", product);

    let q1 = Quaternion::new(1.0, 2.0, 3.0, 4.0);
    let q2 = Quaternion::new(2.0, 3.0, 4.0, 5.0);

    let sum = q1 + q2;
    println!("Sum: {:?}", sum);

    let difference = q2 - q1;
    println!("Difference: {:?}", difference);

    let product = q1 * q2;
    println!("Product: {:?}", product);

    let identity = Quaternion::identity();
    println!("Identity: {:?}", identity);

    let conjugate = q1.conjugate();
    println!("Conjugate: {:?}", conjugate);

    let mut normalized = q1;
    normalized.normalize();
    println!("Normalized: {:?}", normalized);
}
