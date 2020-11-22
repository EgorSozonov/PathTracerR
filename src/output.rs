use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

pub fn createBMP(data: &[u8], w: usize, h: usize, fName: String) -> Result<(), Error> {
    let mut file = File::create(fName)?;

    let fileSize = 3*w*h + 54;
    let mut img: Vec<u8> = vec![0; fileSize];
    for i in 0..w {
        for j in 0..h {
            let indSource: usize = 3*(j*w + 1);
            let indTarget: usize = 3*(j*w + w - i + 1);
            img[indTarget    ] = data[indSource + 2];
            img[indTarget + 1] = data[indSource + 1];
            img[indTarget + 2] = data[indSource    ];
        }
    }
    let mut bmpFileHeader: [u8; 14] = [ 66, 77, 0,0,0,0, 0,0,0,0, 54,0,0,0 ];
    bmpFileHeader[ 2] = (fileSize         & 0xff) as u8;
    bmpFileHeader[ 3] = ((fileSize >>  8) & 0xff) as u8;
    bmpFileHeader[ 4] = ((fileSize >> 16) & 0xff) as u8;
    bmpFileHeader[ 5] = ((fileSize >> 24) & 0xff) as u8;   

    let mut bmpInfoHeader: [u8; 40] = [ 40,0,0,0, 0,0,0,0, 0,0,0,0, 1,0,24,0, 0,0,0,0,
                                         0,0,0,0, 0,0,0,0, 0,0,0,0,  0,0,0,0, 0,0,0,0, ];
    bmpInfoHeader[ 4] = ( w        & 0xff) as u8;
    bmpInfoHeader[ 5] = ((w >>  8) & 0xff) as u8;
    bmpInfoHeader[ 6] = ((w >> 16) & 0xff) as u8;
    bmpInfoHeader[ 7] = ((w >> 24) & 0xff) as u8;
    bmpInfoHeader[ 8] = ( h        & 0xff) as u8;
    bmpInfoHeader[ 9] = ((h >>  8) & 0xff) as u8;
    bmpInfoHeader[10] = ((h >> 16) & 0xff) as u8;
    bmpInfoHeader[11] = ((h >> 24) & 0xff) as u8;

    let bmpPad: [u8; 3] = [0, 0, 0];
    file.write(&bmpFileHeader)?;
    file.write(&bmpInfoHeader)?;
    let lenPad = (4 - (w*3)%4)%4;
    if lenPad > 0 {
        for i in 0..h {
            let slice = &img[w*i*3..w*i*3 + w];
            file.write(slice)?;
            file.write(&bmpPad)?;
        }
    } else {
        for i in 0..h {
            let slice = &img[w*i*3..w*i*3 + w];
            file.write(slice)?;
        }
    }
    file.write_all(&img)?;
    Ok(())
}
