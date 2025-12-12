use ab_glyph::{Font, FontVec, Point, PxScale, ScaleFont};

use std::collections::HashMap;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextVertex {
    pub position: [f32; 2],
    pub uv: [f32; 2],
    pub color: [f32; 4],
}

pub struct TextRenderer {
    pipeline: wgpu::RenderPipeline,
    _bind_group_layout: wgpu::BindGroupLayout,
    atlas_bind_group: wgpu::BindGroup,
    atlas_texture: wgpu::Texture,
    
    font: FontVec,
    atlas: FontAtlas,
    
    vertex_buffer: wgpu::Buffer,
    _vertices: Vec<TextVertex>,
    
    pub queue_buffer: Vec<TextVertex>, // Pending draws
}

struct FontAtlas {
    size: u32,
    cursor: (u32, u32), // x, y
    row_height: u32,
    /// Key is (char, scale_x10) to cache glyphs at different sizes
    glyphs: HashMap<(char, u32), GlyphInfo>,
}

#[derive(Clone, Copy)]
struct GlyphInfo {
    uv_rect: [f32; 4], // u_min, v_min, u_max, v_max
    screen_rect: [f32; 4], // x_off, y_off, w, h
    advance: f32,
}

impl TextRenderer {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, bg_bind_group_layout: &wgpu::BindGroupLayout) -> Self {
        // Load font - assume Segoe UI exists on Windows
        let font_data = std::fs::read("C:/Windows/Fonts/segoeui.ttf").expect("Failed to load generic font");
        let font = FontVec::try_from_vec(font_data).expect("Error parsing font");

        // Create Atlas Texture (R8Unorm)
        let atlas_size = 1024;
        let texture_desc = wgpu::TextureDescriptor {
            label: Some("Font Atlas"),
            size: wgpu::Extent3d { width: atlas_size, height: atlas_size, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        };
        let atlas_texture = device.create_texture(&texture_desc);
        let atlas_view = atlas_texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
             entries: &[
                 wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None },
                 wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None },
            ],
            label: Some("text_atlas_layout"),
        });

        let atlas_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&atlas_view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&sampler) },
            ],
            label: Some("text_atlas_bg"),
        });
        
        // Pipeline
        let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/text.wgsl"));
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Text Pipeline Layout"),
            bind_group_layouts: &[bg_bind_group_layout, &bind_group_layout],
            push_constant_ranges: &[],
        });
        
        let vertex_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<TextVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute { offset: 0, shader_location: 0, format: wgpu::VertexFormat::Float32x2 }, // Pos
                wgpu::VertexAttribute { offset: 8, shader_location: 1, format: wgpu::VertexFormat::Float32x2 }, // UV
                wgpu::VertexAttribute { offset: 16, shader_location: 2, format: wgpu::VertexFormat::Float32x4 }, // Color
            ],
        };

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Text Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState { module: &shader, entry_point: "vs_main", buffers: &[vertex_layout] },
            fragment: Some(wgpu::FragmentState { 
                module: &shader, 
                entry_point: "fs_main", 
                targets: &[Some(wgpu::ColorTargetState { 
                    format: config.format, 
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING), 
                    write_mask: wgpu::ColorWrites::ALL 
                })] 
            }),
            primitive: wgpu::PrimitiveState { topology: wgpu::PrimitiveTopology::TriangleList, ..Default::default() },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });
        
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Text Vertices"),
            size: 1024 * 1024, // 1MB for text
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            pipeline, _bind_group_layout: bind_group_layout, atlas_bind_group, atlas_texture,
            font,
            atlas: FontAtlas { size: atlas_size, cursor: (0, 0), row_height: 0, glyphs: HashMap::new() },
            vertex_buffer,
            _vertices: Vec::new(),
            queue_buffer: Vec::new(),
        }
    }

    pub fn draw_text(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, text: &str, pos: [f32; 2], scale: f32, color: [f32; 4]) {
        let mut x = pos[0];
        let mut y = pos[1];
        let px_scale = PxScale::from(scale);
        let scaled_font = self.font.as_scaled(px_scale);
        
        // Round scale for cache key (multiply by 10 to preserve some precision)
        let scale_key = (scale * 10.0) as u32;
        
        let v_metrics = scaled_font.ascent();
        y += v_metrics; 

        for c in text.chars() {
            if c.is_control() { continue; }
            
            let cache_key = (c, scale_key);
            if !self.atlas.glyphs.contains_key(&cache_key) {
                 self.rasterize_glyph(device, queue, c, px_scale, scale_key);
            }
            
            if let Some(info) = self.atlas.glyphs.get(&cache_key) {
                let w = info.screen_rect[2];
                let h = info.screen_rect[3];
                let gx = x + info.screen_rect[0];
                let gy = y + info.screen_rect[1]; 
                
                let u0 = info.uv_rect[0];
                let v0 = info.uv_rect[1];
                let u1 = info.uv_rect[2];
                let v1 = info.uv_rect[3];
                
                // Quad
                self.queue_buffer.push(TextVertex { position: [gx, gy], uv: [u0, v0], color }); // TL
                self.queue_buffer.push(TextVertex { position: [gx, gy + h], uv: [u0, v1], color }); // BL
                self.queue_buffer.push(TextVertex { position: [gx + w, gy], uv: [u1, v0], color }); // TR
                
                self.queue_buffer.push(TextVertex { position: [gx + w, gy], uv: [u1, v0], color }); // TR
                self.queue_buffer.push(TextVertex { position: [gx, gy + h], uv: [u0, v1], color }); // BL
                self.queue_buffer.push(TextVertex { position: [gx + w, gy + h], uv: [u1, v1], color }); // BR
                
                x += info.advance;
            }
        }
    }
    
    fn rasterize_glyph(&mut self, _device: &wgpu::Device, queue: &wgpu::Queue, c: char, scale: PxScale, scale_key: u32) {
        let glyph = self.font.glyph_id(c).with_scale_and_position(scale, Point { x: 0.0, y: 0.0 });
        let scaled_font = self.font.as_scaled(scale);
        let cache_key = (c, scale_key);
        
        if let Some(outlined) = self.font.outline_glyph(glyph) {
            let bounds = outlined.px_bounds();
            let w = bounds.width() as u32;
            let h = bounds.height() as u32;
            
            if w > 0 && h > 0 {
                // Atlas fit check simpl
                if self.atlas.cursor.0 + w >= self.atlas.size {
                    self.atlas.cursor.0 = 0;
                    self.atlas.cursor.1 += self.atlas.row_height + 2; 
                    self.atlas.row_height = 0;
                }
                
                if self.atlas.cursor.1 + h < self.atlas.size {
                    let mut pixels = vec![0u8; (w * h) as usize];
                    outlined.draw(|x, y, v| {
                       let idx = (y * w + x) as usize;
                       if idx < pixels.len() {
                           pixels[idx] = (v * 255.0) as u8;
                       }
                    });
                    
                    queue.write_texture(
                        wgpu::ImageCopyTexture {
                            texture: &self.atlas_texture,
                            mip_level: 0,
                            origin: wgpu::Origin3d { x: self.atlas.cursor.0, y: self.atlas.cursor.1, z: 0 },
                            aspect: wgpu::TextureAspect::All,
                        },
                        &pixels,
                        wgpu::ImageDataLayout {
                            offset: 0,
                            bytes_per_row: Some(w),
                            rows_per_image: Some(h),
                        },
                        wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
                    );
                    
                    let u0 = self.atlas.cursor.0 as f32 / self.atlas.size as f32;
                    let v0 = self.atlas.cursor.1 as f32 / self.atlas.size as f32;
                    let u1 = (self.atlas.cursor.0 + w) as f32 / self.atlas.size as f32;
                    let v1 = (self.atlas.cursor.1 + h) as f32 / self.atlas.size as f32;
                    
                    self.atlas.glyphs.insert(cache_key, GlyphInfo {
                        uv_rect: [u0, v0, u1, v1],
                        screen_rect: [bounds.min.x, bounds.min.y, w as f32, h as f32],
                        advance: scaled_font.h_advance(self.font.glyph_id(c)),
                    });
                    
                    self.atlas.cursor.0 += w + 2;
                    self.atlas.row_height = self.atlas.row_height.max(h);
                }
            }
        } else {
             self.atlas.glyphs.insert(cache_key, GlyphInfo {
                uv_rect: [0.0; 4],
                screen_rect: [0.0; 4],
                advance: scaled_font.h_advance(self.font.glyph_id(c)),
            });
        }
    }
    
    pub fn prepare(&mut self, queue: &wgpu::Queue) {
        if !self.queue_buffer.is_empty() {
             queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&self.queue_buffer));
        }
    }
    
    pub fn render_range<'a>(&'a self, rpass: &mut wgpu::RenderPass<'a>, bg_bind_group: &'a wgpu::BindGroup, range: std::ops::Range<u32>) {
        if self.queue_buffer.is_empty() || range.start >= range.end { return; }
        
        rpass.set_pipeline(&self.pipeline);
        rpass.set_bind_group(0, bg_bind_group, &[]);
        rpass.set_bind_group(1, &self.atlas_bind_group, &[]);
        rpass.set_vertex_buffer(0, self.vertex_buffer.slice(0..(self.queue_buffer.len() * std::mem::size_of::<TextVertex>()) as u64));
        rpass.draw(range, 0..1);
    }
    
    pub fn clear(&mut self) {
        self.queue_buffer.clear();
    }
}
