#[cfg(test)]
mod tests {
    use moseiik::main::*;
    use image::{
        io::Reader as ImageReader,
    };

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn test_x86() {
        let o = Options{
            image:String::from("assets/kit.jpeg"),
            output:String::from("assets/result-x86.png"),
            tiles:String::from("assets/images"),
            scaling: 1,
            tile_size: 25,
            remove_used:false,
            verbose:true,
            simd:true,
            num_thread:1,
        };
        compute_mosaic(o);
        let img1 = ImageReader::open("./assets/ground-truth-kit-x86-1.png").unwrap().decode().unwrap().to_rgb8();
        let img2 = ImageReader::open("./assets/result-x86.png").unwrap().decode().unwrap().to_rgb8();
        assert_eq!(img1, img2);
    }

    #[test]
    #[cfg(target_arch = "aarch64")]
    fn test_aarch64() {
        let o = Options{
            image:String::from("assets/kit.jpeg"),
            output:String::from("assets/result-aarch64.png"),
            tiles:String::from("assets/images"),
            scaling: 1,
            tile_size: 25,
            remove_used:false,
            verbose:false,
            simd:true,
            num_thread,
        };
        compute_mosaic(o);
        let img1 = ImageReader::open("./assets/ground-truth-kit-arm.png").unwrap().decode().unwrap().to_rgb8();
        let img2 = ImageReader::open("./assets/result-aarch64.png").unwrap().decode().unwrap().to_rgb8();
        assert_eq!(img1, img2);
    }

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn test_generic() {
        let o = Options{
            image:String::from("assets/kit.jpeg"),
            output:String::from("assets/result-generic.png"),
            tiles:String::from("assets/images"),
            scaling: 1,
            tile_size: 25,
            remove_used:false,
            verbose:true,
            simd:false,
            num_thread:1,
        };
        compute_mosaic(o);
        let img1 = ImageReader::open("./assets/ground-truth-kit-x86-1.png").unwrap().decode().unwrap().to_rgb8();
        let img2 = ImageReader::open("./assets/result-generic.png").unwrap().decode().unwrap().to_rgb8();
        assert_eq!(img1, img2);
    }

    #[test]
    #[cfg(target_arch = "aarch64")]
    fn test_generic() {
        let o = Options{
            image:String::from("assets/kit.jpeg"),
            output:String::from("assets/result-generic.png"),
            tiles:String::from("assets/images"),
            scaling: 1,
            tile_size: 25,
            remove_used:false,
            verbose:false,
            simd:false,
            num_thread:1,
        };
        compute_mosaic(o);
        let img1 = ImageReader::open("./assets/ground-truth-kit-arm.png").unwrap().decode().unwrap().to_rgb8();
        let img2 = ImageReader::open("./assets/result-generic.png").unwrap().decode().unwrap().to_rgb8();
        assert_eq!(img1, img2);
    }
}
