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

trait _Draw {
    fn draw_object(&self);
}

trait _Shape: _Draw {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32 {
        println!("Perimeter not implemented for this shape");
        0.0
    }
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
