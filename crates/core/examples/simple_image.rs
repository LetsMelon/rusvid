use rusvid_core::holder::image_holder::ImageHolder;
use rusvid_core::holder::likes::types_like::TypesLike;
use rusvid_core::holder::object::Object;
use rusvid_core::plane::Plane;
use rusvid_core::point::Point;

fn main() {
    let png = image::io::Reader::open("crates/core/examples/cat.jpg")
        .unwrap()
        .decode()
        .unwrap();
    let image = Plane::from_dynamic_image(png).unwrap();

    let object = Object::new(TypesLike::Image(ImageHolder::new_fit(
        Point::new(150.0, 75.0),
        image,
    )));

    let plane = object.render(500, 500).unwrap();
    plane.save_as_png("simple_image.png").unwrap();
}
