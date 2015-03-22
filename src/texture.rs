use glium_lib::texture::Texture2d;
use glium_lib::Display;
use texture_lib::{ TextureWithDevice, ImageSize, TexResult, TexError };

pub struct GliumTexture {
    tex2d: Texture2d,
    width: u32,
    height: u32
}

impl GliumTexture {
    #[inline(always)]
    pub fn get_texture2d(&self) -> &Texture2d {
        &self.tex2d
    }
}

impl TextureWithDevice for GliumTexture {
    type Device = Display;

    fn from_memory(display: &mut <GliumTexture as TextureWithDevice>::Device,
                   memory: &[u8], width: usize, channels: usize) -> TexResult<Self> {
        let tex2d = match channels {
            1 => Texture2d::with_mipmaps::<Vec<Vec<(u8, u8, u8, u8)>>>(
                     display,
                     memory.chunks(width)
                           .map(|row| row.iter().map(|&p| (255, 255, 255, p)).collect())
                           .rev().collect(),
                     false
                 ),
            4 => Texture2d::with_mipmaps::<Vec<Vec<(u8, u8, u8, u8)>>>(
                     display,
                     memory.chunks(width * channels)
                           .map(|row| row.chunks(4).map(|p| (p[0], p[1], p[2], p[3])).collect())
                           .rev().collect(),
                     false
                 ),
            n => return Err(TexError::Channels(n))
        };
        let height = memory.len() / width / channels;
        Ok(GliumTexture {
            tex2d: tex2d,
            width: width as u32,
            height: height as u32
        })
    }

    fn update_from_memory(&mut self, display: &mut <GliumTexture as TextureWithDevice>::Device,
                          memory: &[u8], width: usize, channels: usize) -> TexResult<()> {
        Ok(()) // TODO
    }
}

impl ImageSize for GliumTexture {
    #[inline(always)]
    fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
