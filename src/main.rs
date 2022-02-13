fn murmur_add(hash: u32, element: u32) -> u32 {
    let mut element = element;
    let mut hash = hash;

    element = element.wrapping_mul(0xcc9e2d51u32);
    element = (element << 15) | (element >> (32 - 15));
    element = element.wrapping_mul(0x1b873593u32);
    hash ^= element;
    hash = (hash << 13) | (hash >> (32 - 13));
    hash = hash.wrapping_mul(5u32).wrapping_add(0xe6546b64);
    return hash;
}

fn murmur_mix(hash: u32) -> u32 {
    let mut hash = hash;
    hash ^= hash.wrapping_shr(16u32);
    hash = hash.wrapping_mul(0x85ebca6bu32);
    hash ^= hash.wrapping_shr(13u32);
    hash = hash.wrapping_mul(0xc2b2ae35u32);
    hash ^= hash.wrapping_shr(16u32);
    return hash;
}

pub struct HashTable {
    data: Vec<u32>,
}

impl HashTable {
    pub fn new(size: usize) -> Self {
        let mut data = Vec::with_capacity(size);
        data.resize(size, 0);
        let result = HashTable { data: data };
        return result;
    }

    pub fn add(&mut self, key: u32) -> (bool, u32) {
        let key = key + 1;
        let mut index = murmur_mix(key);
        loop {
            index = index % self.data.len() as u32;
            let stored_key = self.data[index as usize];
            if stored_key != key {
                if stored_key != 0 {
                    index += 1;
                    continue;
                } else {
                    self.data[index as usize] = key;
                    return (true, index);
                }
            }
            break;
        }

        return (false, index);
    }

    pub fn find(&self, key: u32) -> (bool, u32) {
        let key = key + 1;
        let mut index = murmur_mix(key);
        loop {
            index = index % self.data.len() as u32;
            let stored_key = self.data[index as usize];
            if stored_key != key {
                if stored_key != 0 {
                    index += 1;
                    continue;
                }

                return (true, index);
            }

            break;
        }

        return (false, index);
    }
}

pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

fn encode_voxel_key(pos: f32) -> u32 {
    let signed = pos as i32 + 0x1000000;
    let voxel = signed as u32 & 0xfffffff;
    return voxel;
}

fn main() {
    const HASH_TABLE_SIZE: usize = 128;
    let mut hash_table = HashTable::new(HASH_TABLE_SIZE);
    let mut hash_table_value: Vec<f32> = Vec::with_capacity(HASH_TABLE_SIZE);
    hash_table_value.resize(HASH_TABLE_SIZE, 0.0);

    let a = Vec3{x:0.0, y:10.0, z:18.0};
    let mut hash_a:u32 = 0;
    hash_a = murmur_add(hash_a, encode_voxel_key(a.x));
    hash_a = murmur_add(hash_a, encode_voxel_key(a.y));
    hash_a = murmur_add(hash_a, encode_voxel_key(a.z));
    let (_, bucket_a) = hash_table.add(hash_a);
    println!("bucket is {}", bucket_a);
    let a_radiance:f32 = 90.0;
    hash_table_value[bucket_a as usize] = a_radiance;

    let b = Vec3{x:-8.0, y:102.0, z:35.0};
    let mut hash_b:u32 = 0;
    hash_b = murmur_add(hash_b, encode_voxel_key(b.x));
    hash_b = murmur_add(hash_b, encode_voxel_key(b.y));
    hash_b = murmur_add(hash_b, encode_voxel_key(b.z));
    let (_, bucket_b) = hash_table.add(hash_b);
    println!("bucket is {}", bucket_b);
    let b_radiance:f32 = 20.0;
    hash_table_value[bucket_b as usize] = b_radiance;
}
