struct _Square {
    _side: f32,
    _line_width: u8,
    _color: String,
}

struct _Rectangle {
    _length: f32,
    _width: f32,
    _line_width: u8,
    _color: String,
}

trait _OtherTrait {
    fn some_other_method(&self);
}
trait _SomeOtherTrait {}

trait _Draw {
    fn draw_object(&self);
}

trait _Shape: _Draw + _OtherTrait + _SomeOtherTrait {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32 {
        println!("Perimeter not implemented for this shape");
        0.0
    }
}

fn _shape_properties<T>(object: T)
where
    // T: _Shape + _OtherTrait + _SomeOtherTrait,
    T: _Shape,
{
    object.area();
    object.perimeter();
    object.draw_object();
}

impl _Draw for _Square {
    fn draw_object(&self) {
        println!(
            "Drawing square with side: {}, line width: {}, color: {}",
            self._side, self._line_width, self._color
        );
    }
}

impl _Draw for _Rectangle {
    fn draw_object(&self) {
        println!(
            "Drawing rectangle with length: {}, width: {}, line width: {}, color: {}",
            self._length, self._width, self._line_width, self._color
        );
    }
}

impl _Shape for _Square {
    fn area(&self) -> f32 {
        self._side * self._side
    }
}

impl _Shape for _Rectangle {
    fn area(&self) -> f32 {
        self._length * self._width
    }
    fn perimeter(&self) -> f32 {
        2.0 * (self._length + self._width)
    }
}

fn main() {}
