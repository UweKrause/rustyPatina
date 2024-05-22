use image::io::Reader as ImageReader;


fn main() {
    let img = ImageReader::open("landscape.jpg").expect("PROBLEM").decode();
    let blurred = img.expect("PROBLEM 2").blur(5.5);
    blurred.save("landscape_blurred.png").expect("PROBLEM 3");
}
