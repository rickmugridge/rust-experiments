#[derive(Debug, PartialEq, Clone)]
struct Graphics {
    entries: Vec<usize>,
}

impl Graphics {
    fn new() -> Self {
        Self { entries: vec![] }
    }

    fn draw(&mut self, i: usize) {
        self.entries.push(i);
    }
}

trait Shape {
    fn draw(&self, g: &mut Graphics);
}

struct Line {
    start: usize,
}

impl Line {
    fn new(start: usize) -> Self {
        Self { start }
    }
}

impl Shape for Line {
    fn draw(&self, g: &mut Graphics) {
        g.draw(self.start);
    }
}

struct Lines {
    lines: Vec<Line>
}

// impl Lines {
//     fn new() -> Self {
//         Self { lines: vec![] }
//     }
// }

impl Shape for Lines {
    fn draw(&self, g: &mut Graphics) {
        // for line in &self.lines {g.draw(line.start);}
        for line in &self.lines {line.draw(g);} // todo Note how we can't say (&mut g)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrow() {
        let mut g = Graphics::new();
        let line = Line::new(8);
        line.draw(&mut g);
        line.draw(&mut g);
        assert_eq!(g.entries, vec![8, 8]);
    }
}