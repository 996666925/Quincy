use thunderdome::{Arena, Generation, Index};

pub struct IndexExt;

impl IndexExt {
    pub fn to_rgba(index: Index) -> [u8; 4] {
        let num = index.slot() << 16 | index.generation();
        let binding = [num];
        let nums: &[u8] = bytemuck::cast_slice(&binding);

        [nums[0], nums[1], nums[2], nums[3]]
    }

    pub fn to_rgba_f32(index: Index) -> [f32; 4] {
        let num = index.slot() << 16 | index.generation();
        let binding = [num];
        let nums: &[u8] = bytemuck::cast_slice(&binding);

        [
            nums[0] as f32 / 255.,
            nums[1] as f32 / 255.,
            nums[2] as f32 / 255.,
            nums[3] as f32 / 255.,
        ]
    }

    pub fn u64_to_index(num: u64) -> Index {
        let binding = [num];
        let bytes: &[u16] = bytemuck::cast_slice(&binding);

        Index {
            generation: Generation::from_u32(bytes[0] as _).unwrap(),
            slot: bytes[1] as _,
        }
    }

    pub fn u8_to_index(nums: [u8; 4]) -> Index {
        let bytes: &[u16] = bytemuck::cast_slice(&nums);


        Index {
            generation: Generation::from_u32(bytes[0] as _).unwrap(),
            slot: bytes[1] as _,
        }
    }

    pub fn f32_to_index(nums: [f32; 4]) -> Index {
        let nums: [u8; 4] = [
            (nums[0] * 255.) as u8,
            (nums[1] * 255.) as u8,
            (nums[2] * 255.) as u8,
            (nums[3] * 255.) as u8,
        ];

        Self::u8_to_index(nums)
    }
}

#[test]
fn test() {
    let num = 123 << 16 | 123;

    let mut arena = Arena::new();
    let mut index = Index::DANGLING;
    for i in 0..50000 {
        index = arena.insert(666666);
    }
    println!("{:?}", num);

    let nums = IndexExt::to_rgba(index);
    println!("u8:{:?}", nums);

    let num = IndexExt::u8_to_index(nums);

    println!("u8:{:?}", num);

    let nums = IndexExt::to_rgba_f32(index);
    println!("f32:{:?}", nums);

    let num = IndexExt::f32_to_index(nums);

    println!("f32:{:?}", num);
}
