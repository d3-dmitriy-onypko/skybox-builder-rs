use std::{env, fs, io::Error, path::PathBuf};

use image::{GenericImage, GenericImageView, ImageBuffer};

const SKYBOX_TILES_AMOUNT: usize = 6;

pub fn merge_all_files() -> Result<(), Error> {
    let file_paths = get_file_paths()?;
    for skybox in get_skyboxes(file_paths) {
        println!("Processing skybox tiles");
        dbg!(&skybox);
        skybox.merge();
    }

    Ok(())
}

fn get_file_paths() -> Result<Vec<PathBuf>, Error> {
    let path = env::current_dir().expect("Should be able to read current directory");

    println!("Processign dir {}", path.display());

    let paths: Vec<PathBuf> = fs::read_dir(path)?
        .filter_map(Result::ok)
        .map(|f| f.path())
        .filter(|f| f.is_file() && f.extension().unwrap_or_default() == "png")
        .collect();

    print!("Found {} files", paths.len());

    Ok(paths)
}

fn get_skyboxes(paths: Vec<PathBuf>) -> Vec<SkyBoxFiles> {
    match paths.len() {
        s if s > SKYBOX_TILES_AMOUNT => panic!("Directory should have only {} tile files for now as merging of single skybox is supported", SKYBOX_TILES_AMOUNT),
        s if s < SKYBOX_TILES_AMOUNT => panic!("Ensure all skybox tiles are present"),
        _ => {}
    }

    let tiles: Vec<SkyboxTile> = paths
        .into_iter()
        .map(|path| match path.file_name().and_then(|f| f.to_str()) {
            Some(p) if p.ends_with("left.png") => Some(SkyboxTile {
                path,
                position: SkyboxTilePosition::Left,
            }),
            Some(p) if p.ends_with("right.png") => Some(SkyboxTile {
                path,
                position: SkyboxTilePosition::Right,
            }),
            Some(p) if p.ends_with("up.png") => Some(SkyboxTile {
                path,
                position: SkyboxTilePosition::Up,
            }),
            Some(p) if p.ends_with("down.png") => Some(SkyboxTile {
                path,
                position: SkyboxTilePosition::Down,
            }),
            Some(p) if p.ends_with("front.png") => Some(SkyboxTile {
                path,
                position: SkyboxTilePosition::Front,
            }),
            Some(p) if p.ends_with("back.png") => Some(SkyboxTile {
                path,
                position: SkyboxTilePosition::Back,
            }),
            Some(_) | None => None,
        })
        .flatten()
        .collect();

    vec![SkyBoxFiles { tiles }]
}

#[derive(Debug)]
struct SkyBoxFiles {
    tiles: Vec<SkyboxTile>,
}

#[derive(PartialEq, Debug)]
struct SkyboxTile {
    path: PathBuf,
    position: SkyboxTilePosition,
}

#[derive(PartialEq, Debug)]
enum SkyboxTilePosition {
    Left,
    Right,
    Up,
    Down,
    Front,
    Back,
}

impl SkyBoxFiles {
    fn merge(self) {
        if self.tiles.len() != SKYBOX_TILES_AMOUNT {
            eprintln!("Not all tiles are set for skybox. Skipping skybox");
            return;
        }

        let first_file = image::open(&self.tiles[0].path)
            .expect("First tile should be opened to calculate dimensions");
        let (width, height) = first_file.dimensions();

        drop(first_file);

        let mut result_file = ImageBuffer::new(width * 4, height * 3);

        for tile in self.tiles.into_iter() {
            let pic = image::open(tile.path).unwrap();

            match tile.position {
                SkyboxTilePosition::Left => result_file
                    .copy_from(&pic, 0, height)
                    .expect("skybox tile copy success"),
                SkyboxTilePosition::Right => result_file
                    .copy_from(&pic, width * 2, height)
                    .expect("skybox tile copy success"),
                SkyboxTilePosition::Up => result_file
                    .copy_from(&pic, width, 0)
                    .expect("skybox tile copy success"),
                SkyboxTilePosition::Down => result_file
                    .copy_from(&pic, width, height * 2)
                    .expect("skybox tile copy success"),
                SkyboxTilePosition::Front => result_file
                    .copy_from(&pic, width, height)
                    .expect("skybox tile copy success"),
                SkyboxTilePosition::Back => result_file
                    .copy_from(&pic, width * 3, height)
                    .expect("skybox tile copy success"),
            };
        }

        result_file
            .save_with_format("skybox.png", image::ImageFormat::Png)
            .expect("File saved");
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn get_skyboxes_group_single_file() {
        let l_path = PathBuf::from("skybox_01a_left.png");
        let r_path = PathBuf::from("skybox_01a_right.png");
        let u_path = PathBuf::from("skybox_01a_up.png");
        let d_path = PathBuf::from("skybox_01a_down.png");
        let f_path = PathBuf::from("skybox_01a_front.png");
        let b_path = PathBuf::from("skybox_01a_back.png");

        //assert!(l_path.file_name().unwrap().to_owned().into_string().unwrap().ends_with("left.png"));

        let paths = vec![
            l_path.clone(),
            r_path.clone(),
            u_path.clone(),
            d_path.clone(),
            f_path.clone(),
            b_path.clone(),
        ];

        // let expected = SkyBoxFiles {
        //     left: Some(l_path),
        //     right: Some(r_path),
        //     up: Some(u_path),
        //     down: Some(d_path),
        //     front: Some(f_path),
        //     back: Some(b_path),
        // };

        // let skyboxes = get_skyboxes(paths);

        // assert!(skyboxes.len() == 1);

        // assert_eq!(*skyboxes.first().unwrap(), expected);
    }
}
