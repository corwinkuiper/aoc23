static INPUT: &str = include_str!("input.txt");
static TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

fn hash_bytes(b: &[u8]) -> u8 {
    b.iter()
        .copied()
        .fold(0u8, |acc, b| acc.wrapping_add(b).wrapping_mul(17))
}

fn first_task(input: &str) -> u64 {
    input
        .split(',')
        .map(|s| {
            s.as_bytes()
                .iter()
                .copied()
                .fold(0u8, |acc, b| acc.wrapping_add(b).wrapping_mul(17))
        })
        .map(|x| Into::<u64>::into(x))
        .sum()
}

enum Operation {
    Add(u32),
    Remove,
}

struct Action<'a> {
    label: &'a [u8],
    operation: Operation,
}

struct Lens<'a> {
    label: &'a [u8],
    focal_length: u32,
}

fn second_task(input: &str) -> u64 {
    let mut map = Vec::new();
    for _ in 0..256 {
        map.push(Vec::<Lens>::new());
    }

    let actions = input.split(',').map(|s| {
        let (label, operation) = {
            if let Some((label, focal_length)) = s.split_once('=') {
                (label, Operation::Add(focal_length.parse().unwrap()))
            } else {
                let (label, end) = s.split_once('-').unwrap();
                assert_eq!(end, "");
                (label, Operation::Remove)
            }
        };
        Action {
            label: label.as_bytes(),
            operation,
        }
    });

    for Action { label, operation } in actions {
        let hash = hash_bytes(label) as usize;

        let v = &mut map[hash];

        match operation {
            Operation::Add(focal_length) => {
                match v.iter_mut().filter(|x| x.label == label).next() {
                    Some(lens) => lens.focal_length = focal_length,
                    None => v.push(Lens {
                        label,
                        focal_length,
                    }),
                }
            }
            Operation::Remove => match v
                .iter_mut()
                .enumerate()
                .filter(|(_, x)| x.label == label)
                .next()
            {
                Some((idx, _)) => {
                    v.remove(idx);
                }
                None => {}
            },
        }
    }

    map.iter()
        .zip(1u64..)
        .map(|(lens_box, idx)| {
            lens_box
                .iter()
                .zip(1u64..)
                .map(|(lens, lens_idx)| lens.focal_length as u64 * lens_idx * idx)
                .sum::<u64>()
        })
        .sum()
}

#[test]
fn check_first_task() {
    assert_eq!(first_task(TEST_INPUT), 1320);
}

#[test]
fn check_second_task() {
    assert_eq!(second_task(TEST_INPUT), 145);
}

fn main() {
    dbg!(first_task(INPUT));
    dbg!(second_task(INPUT));
}
