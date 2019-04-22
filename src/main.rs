mod image;
mod math;

fn main() {
    // Image test
    let image = image::Image::new(20, 20);
    image.write("test.ppm");

    // Vector test
    let vector = math::Vector::new(3.0, 2.0, 1.0);
    let vector2 = math::Vector::new(7.0, 4.0, 3.0);
    let sum_vector = vector + vector2;
    let sub_vector = vector - vector2;
    let dotted = math::Vector::dot_product(vector, vector2);
    println!(
        "Sum vector: {} {} {}",
        sum_vector.x, sum_vector.y, sum_vector.z
    );

    println!(
        "Sub vector: {} {} {}",
        sub_vector.x, sub_vector.y, sub_vector.z
    );

    println!("Dot product result: {}", dotted);
}
