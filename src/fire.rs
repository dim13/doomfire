use enum_derived::Rand;

const WIDTH: usize = 320;
const HEIGHT: usize = 200;

pub struct Fire<'a> {
    pixels: Vec<u8>,
    palette: &'a [u32],
}

#[derive(Rand)]
enum Jitter {
    Left,
    Right,
    Middle,
}

impl Fire<'_> {
    pub fn new() -> Self {
        Fire {
            pixels: vec![0u8; WIDTH * HEIGHT],
            palette: PALETTE,
        }
    }
    pub fn size(&self) -> (usize, usize) {
        (WIDTH, HEIGHT)
    }
    pub fn seed(&mut self) {
        let y = HEIGHT - 1;
        let c = self.palette.len() as u8 - 1;
        for x in 0..WIDTH {
            self.set_at(x, y, c);
        }
    }
    pub fn bytes(&mut self) -> Vec<u32> {
        let mut buffer = vec![0u32; WIDTH * HEIGHT];
        for (n, v) in self.pixels.iter().enumerate() {
            buffer[n] = self.palette[*v as usize];
        }
        buffer
    }
    pub fn update(&mut self) {
        const MAX_WIDTH: usize = WIDTH - 1;
        for x in 0..WIDTH {
            for y in 1..HEIGHT {
                let z = Jitter::rand();
                let c = self.get_at(x, y);
                let (x, c) = match (z, x, c) {
                    (Jitter::Left, 1.., _) => (x - 1, c),
                    (Jitter::Middle, _, 1..) => (x, c - 1),
                    (Jitter::Right, ..MAX_WIDTH, _) => (x + 1, c),
                    _ => (x, c),
                };
                self.set_at(x, y - 1, c);
            }
        }
    }
    fn index(&self, x: usize, y: usize) -> usize {
        WIDTH * y + x
    }
    fn get_at(&self, x: usize, y: usize) -> u8 {
        let n = self.index(x, y);
        self.pixels[n]
    }
    fn set_at(&mut self, x: usize, y: usize, v: u8) {
        let n = self.index(x, y);
        self.pixels[n] = v;
    }
}

const PALETTE: &[u32] = &[
    0x070707, 0x1f0707, 0x2f0f07, 0x470f07, 0x571707, 0x671f07, 0x771f07, 0x8f2707, 0x9f2f07,
    0xaf3f07, 0xbf4707, 0xc74707, 0xdf4f07, 0xdf5707, 0xdf5707, 0xd75f07, 0xd7670f, 0xcf6f0f,
    0xcf770f, 0xcf7f0f, 0xcf8717, 0xc78717, 0xc78f17, 0xc7971f, 0xbf9f1f, 0xbf9f1f, 0xbfa727,
    0xbfa727, 0xbfaf2f, 0xb7af2f, 0xb7b72f, 0xb7b737, 0xcfcf6f, 0xdfdf9f, 0xefefc7, 0xffffff,
];
