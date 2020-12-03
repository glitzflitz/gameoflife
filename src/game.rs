use crate::vga_buffer::{Color, BUFFER_HEIGHT, BUFFER_WIDTH, WRITER};

type Cells = [[bool; BUFFER_WIDTH]; BUFFER_HEIGHT];

pub fn start() -> ! {
    let mut cells = [[false; BUFFER_WIDTH]; BUFFER_HEIGHT];

    let seed = 6567253;
    let mut rng = oorandom::Rand64::new(seed);
    for y in 0..BUFFER_HEIGHT {
        for x in 0..BUFFER_WIDTH {
            cells[y][x] = rng.rand_u64() & 1 == 0;
        }
    }

    loop {
        render(&cells);
        let mut sleep = 1000000;
        while sleep > 0 {
            sleep -= 1;
        }
        step(&mut cells);
    }
}

fn neighbours(cells: &Cells, x: usize, y: usize) -> u8 {
    let mut count = 0u8;
    for c in [
        cells[((x + BUFFER_HEIGHT - 1) % BUFFER_HEIGHT) as usize]
            [((y + BUFFER_WIDTH - 1) % BUFFER_WIDTH) as usize],
        cells[((x + BUFFER_HEIGHT - 1) % BUFFER_HEIGHT) as usize][y as usize],
        cells[((x + BUFFER_HEIGHT - 1) % BUFFER_HEIGHT) as usize]
            [((y + BUFFER_WIDTH + 1) % BUFFER_WIDTH) as usize],
        cells[x as usize][((y + BUFFER_WIDTH - 1) % BUFFER_WIDTH) as usize],
        cells[x as usize][((y + BUFFER_WIDTH + 1) % BUFFER_WIDTH) as usize],
        cells[((x + BUFFER_HEIGHT + 1) % BUFFER_HEIGHT) as usize]
            [((y + BUFFER_WIDTH - 1) % BUFFER_WIDTH) as usize],
        cells[((x + BUFFER_HEIGHT + 1) % BUFFER_HEIGHT) as usize][y as usize],
        cells[((x + BUFFER_HEIGHT + 1) % BUFFER_HEIGHT) as usize]
            [((y + BUFFER_WIDTH + 1) % BUFFER_WIDTH) as usize],
    ]
    .iter()
    {
        if *c {
            count += 1;
        }
    }
    count
}

fn step(cells: &mut Cells) {
    let mut buf = [[false; BUFFER_WIDTH]; BUFFER_HEIGHT];

    let mut y = 0;
    let mut x = 0;

    while y < BUFFER_HEIGHT {
        while x < BUFFER_WIDTH {
            if cells[y][x] {
                match neighbours(&cells, y, x) {
                    2 | 3 => {
                        buf[y][x] = true;
                    }
                    _ => {}
                }
            } else {
                if neighbours(&cells, y, x) == 3 {
                    buf[y][x] = true;
                }
            }
            x += 1;
        }
        x = 0;
        y += 1;
    }
    *cells = buf;
}

fn render(cells: &Cells) {
    for y in 0..BUFFER_HEIGHT {
        for x in 0..BUFFER_WIDTH {
            if cells[y][x] == true {
                let color = Color::Pink;
                WRITER
                    .lock()
                    .write_char(' ' as u8, x, y, Color::White, color);
            } else {
                let color = Color::Black;
                WRITER.lock().write_char(' ' as u8, x, y, color, color);
            }
        }
    }
}
