use lib::prelude::*;

struct CPU {
    ic: usize,
    x: i32,
    str_sum: i32,
}

impl CPU {
    fn new() -> CPU {
        CPU {
            ic: 1,
            x: 1,
            str_sum: 0,
        }
    }
    fn tick(&mut self) {
        if self.ic % 40 == 20 {
            let ic_: i32 = self.ic.try_into().unwrap();
            self.str_sum += self.x * ic_;
        }
        self.ic += 1;
    }
}

pub fn part_one(input: &str) -> Result<i32> {
    let mut cpu = CPU::new();
    for line in input.split('\n') {
        if cpu.ic > 220 {
            break;
        }
        cpu.tick();
        if line != "noop" {
            cpu.tick();
            cpu.x += line.strip_prefix("addx ").unwrap().parse::<i32>().unwrap();
        }
    }

    Ok(cpu.str_sum)
}

fn crt_draw(cpu: &CPU) -> bool {
    if cpu.x < 0 {
        return ((cpu.ic - 1) % 40) == 0;
    }
    let ic: i32 = ((cpu.ic - 1) % 40).try_into().unwrap();
    vec![cpu.x - 1, cpu.x, cpu.x + 1].contains(&ic)
}

pub fn part_two(input: &str) -> Result<String> {
    let mut cpu = CPU::new();
    let mut crt = Vec::with_capacity(240);

    for line in input.split('\n') {
        crt.push(crt_draw(&cpu));
        cpu.tick();

        if line != "noop" {
            crt.push(crt_draw(&cpu));
            cpu.tick();
            cpu.x += line.strip_prefix("addx ").unwrap().parse::<i32>().unwrap();
        }
    }
    let mut res: Vec<char> = Vec::new();
    for (i, pixel) in crt.into_iter().enumerate() {
        if i % 40 == 0 {
            res.push('\n');
        }
        match pixel {
            true => res.push('#'),
            false => res.push(' '),
        }
    }
    Ok(res.into_iter().collect())
}

fn main() -> Result<()> {
    let input = input!("d10.txt")?;

    run!("part_one", part_one(&input)?);
    run!("part_two", part_two(&input)?);
    Ok(())
}

//  and ⬜
