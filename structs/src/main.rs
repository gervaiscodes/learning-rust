struct Foo {
    quax: i32,
    baz: String,
    z: Fuz
}

struct Fuz {
    zed: i32
}

struct Point2D {
    x: f64,
    y: f64
}

fn add_points(a: Point2D, b: Point2D) -> Point2D {
    Point2D {
        x: a.x + b.x,
        y: a.y + b.y
    }
}

fn main() {
    let a = Foo {
        quax: 10,
        baz: String::from("Hello"),
        z: Fuz {
            zed: 4
        }
    };
    println!("Foo: quax={} baz={} zed={}", a.quax, a.baz, a.z.zed);

    let sum = add_points(Point2D {
        x: 232.4324,
        y: 93.43
    }, Point2D {
        x: 23.2,
        y: 8.32
    });
    println!("Point sum => x={} y={}", sum.x, sum.y);
}
