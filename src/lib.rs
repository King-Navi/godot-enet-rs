// #![allow(non_upper_case_globals)]
// #![allow(non_camel_case_types)]
// #![allow(non_snake_case)]
pub mod sys {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
pub mod host;
pub mod packet;
// Re-exportamos para que el usuario pueda hacer:
// use godot_enet_rs::{ENetLibrary, GodotENetHost};
pub use host::{ENetLibrary, GodotENetHost};

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn test_server_creation() {
        let lib = Rc::new(ENetLibrary::new().unwrap());
        let server = GodotENetHost::create_server(&lib, 4444, 32);
        assert!(server.is_ok(), "El servidor debería crearse correctamente");
        println!("✅ Test: Servidor creado en puerto 4444");
    }
}