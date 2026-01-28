use crate::sys;
use std::rc::Rc;
use crate::packet::Packet;
pub struct ENetLibrary;

impl ENetLibrary {
    pub fn new() -> Result<Self, String> {
        let result = unsafe { sys::enet_initialize() };
        if result == 0 {
            Ok(ENetLibrary)
        } else {
            Err("Fallo al inicializar ENet".to_string())
        }
    }
}

impl Drop for ENetLibrary {
    fn drop(&mut self) {
        unsafe { sys::enet_deinitialize() };
    }
}

pub struct GodotENetHost {
    host: *mut sys::ENetHost,
    _lib: Rc<ENetLibrary>, 
}

impl GodotENetHost {
    pub fn create_server(lib: &Rc<ENetLibrary>, port: u16, max_peers: usize) -> Result<Self, String> {
        let mut address = sys::ENetAddress {
            // Godot usa un array de 16 bytes para soportar IPv6.
            // [0; 16] es equivalente a 0.0.0.0 (o :: en IPv6), es decir, "vacío".
            host: [0; 16], 
            
            port: port,
            
            // Este campo es nuevo en Godot. 
            // 1 indica que queremos escuchar en TODAS las interfaces de red (Wildcard).
            wildcard: 1, 
        };
        // Valores por defecto típicos de Godot
        let channel_limit = 32; 
        let incoming_bandwidth = 0;
        let outgoing_bandwidth = 0;

        let host_ptr = unsafe {
            sys::enet_host_create(
                &mut address,
                max_peers as _,
                channel_limit,
                incoming_bandwidth,
                outgoing_bandwidth,
            )
        };

        if host_ptr.is_null() {
            return Err("No se pudo crear el ENet Host".to_string());
        }

        Ok(GodotENetHost {
            host: host_ptr,
            _lib: lib.clone(),
        })
    }

    pub fn service(&mut self, timeout_ms: u32) -> Result<Event, String> {
        let mut event: sys::ENetEvent = unsafe { std::mem::zeroed() };
        
        let result = unsafe {
            sys::enet_host_service(self.host, &mut event, timeout_ms)
        };

        if result < 0 {
            return Err("Error en enet_host_service".to_string());
        }

        if result == 0 {
            return Ok(Event::None);
        }

        // Convertir el evento de C a Rust
        match event.type_ {
            sys::_ENetEventType_ENET_EVENT_TYPE_CONNECT => {
                // Obtenemos el ID del peer conectado (útil para Godot)
                let peer = unsafe { &*event.peer };
                Ok(Event::Connect { peer_id: peer.incomingPeerID as u32 })
            },
            sys::_ENetEventType_ENET_EVENT_TYPE_DISCONNECT => {
                let peer = unsafe { &*event.peer };
                Ok(Event::Disconnect { 
                    peer_id: peer.incomingPeerID as u32,
                    data: event.data 
                })
            },
            sys::_ENetEventType_ENET_EVENT_TYPE_RECEIVE => {
                let peer = unsafe { &*event.peer };
                // IMPORTANTE: ENet nos da ownership del packet aquí.
                // Lo envolvemos en nuestro struct para que Rust lo libere después.
                let packet = Packet { raw: event.packet };
                
                Ok(Event::Receive {
                    peer_id: peer.incomingPeerID as u32,
                    channel_id: event.channelID,
                    packet,
                })
            },
            _ => Ok(Event::None),
        }
    }
}

impl Drop for GodotENetHost {
    fn drop(&mut self) {
        if !self.host.is_null() {
            unsafe { sys::enet_host_destroy(self.host) };
        }
    }
}

#[derive(Debug)]
pub enum Event {
    Connect { peer_id: u32 }, // Godot usa el peer_id, no solo el puntero
    Disconnect { peer_id: u32, data: u32 },
    Receive { 
        peer_id: u32, 
        channel_id: u8, 
        packet: Packet 
    },
    None,
}