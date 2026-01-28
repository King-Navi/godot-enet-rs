use crate::sys;


#[derive(Debug)]
pub struct Packet {
    pub(crate) raw: *mut sys::ENetPacket,
}

impl Packet {
    /// Crea un paquete copiando los datos del slice proporcionado.
    /// flags: 1 = Reliable, 0 = Unreliable (Estándar ENet)
    pub fn new(data: &[u8], flags: u32) -> Result<Self, String> {
        let packet = unsafe {
            sys::enet_packet_create(
                data.as_ptr() as *const _,
                data.len(),
                flags,
            )
        };

        if packet.is_null() {
            Err("No se pudo crear el paquete (Memoria insuficiente?)".to_string())
        } else {
            Ok(Packet { raw: packet })
        }
    }

    /// Obtiene los datos del paquete como un slice seguro de Rust
    pub fn data(&self) -> &[u8] {
        unsafe {
            let p = &*self.raw;
            std::slice::from_raw_parts(p.data, p.dataLength)
        }
    }
}

impl Drop for Packet {
    fn drop(&mut self) {
        unsafe { sys::enet_packet_destroy(self.raw) };
    }
}